use anyhow::Context;
use clap::ValueEnum;
use reqwest::Client;
use serde::{Deserialize, Serialize};

static DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.3";

#[derive(Serialize, Deserialize, Debug, Default, sqlx::FromRow)]
pub(crate) struct Link {
    #[serde(default)]
    pub id: i32,
    pub title: String,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub url: String,
}

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum PrintFormat {
    Jsonl,
    Tsv,
    Pretty,
}

impl Link {
    pub fn new(url: &str, title: &str) -> anyhow::Result<Self> {
        let url = url::Url::parse(&url).with_context(|| format!("could not parse url: {url}"))?;

        if !url.has_host() {
            anyhow::bail!("invalid url")
        }

        let link: Self = Link {
            title: title.replace("\t", " "),
            url: url.host_str().unwrap().to_string(),
            ..Default::default()
        };
        Ok(link)
    }

    pub async fn from_url(url: &str) -> anyhow::Result<Self> {
        let url = url::Url::parse(&url).with_context(|| format!("could not parse url: {url}"))?;
        let mut link: Self = Link {
            url: url.to_string(),
            ..Default::default()
        };

        if !url.has_host() {
            anyhow::bail!("invalid url: {}", url.as_str())
        }

        // create http client
        let hc = Client::builder()
            .user_agent(std::env::var("LI_USERAGENT").unwrap_or(DEFAULT_USER_AGENT.to_string()))
            .build()
            .context("reqwest client build error")?;

        // send http request to the url
        let res = hc
            .get(url.as_str())
            .send()
            .await
            .with_context(|| format!("request error: {}", url.as_str()))?;

        // read body to buffer
        let buf = res
            .text()
            .await
            .context("could not read the response body")?;
        let doc = scraper::Html::parse_document(&buf);

        // metadata selectors
        let title_selector = scraper::Selector::parse("title").unwrap();
        let desc0_selector = scraper::Selector::parse(r#"meta[name="description"]"#).unwrap();
        let desc1_selector =
            scraper::Selector::parse(r#"meta[property="og:description"]"#).unwrap();
        let desc2_selector =
            scraper::Selector::parse(r#"meta[name="twitter:description"]"#).unwrap();

        // try to parse the title from the html document
        link.title = match doc.select(&title_selector).next() {
            Some(title) => title
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .replace("\t", " "),
            None => url.as_str().to_string(),
        };

        // try to parse the description from the html document
        link.description = vec![
            doc.select(&desc0_selector),
            doc.select(&desc1_selector),
            doc.select(&desc2_selector),
        ]
        .into_iter()
        .filter_map(|mut x| x.next())
        .filter_map(|x| {
            x.attr("content")
                .map(ToString::to_string)
                .filter(|x| !x.is_empty())
        })
        .next();

        Ok(link)
    }

    pub async fn get_link_by_id(
        conn: &mut sqlx::PgConnection,
        id: i32,
    ) -> anyhow::Result<Self, sqlx::Error> {
        let link: Link = sqlx::query_as("SELECT * FROM links WHERE id = $1 LIMIT 1")
            .bind(&id)
            .fetch_one(conn)
            .await?;
        Ok(link)
    }

    pub async fn insert(
        &self,
        conn: &mut sqlx::PgConnection,
    ) -> anyhow::Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO links (url, title, description, tags) VALUES ($1, $2, $3, $4)")
            .bind(&self.url)
            .bind(&self.title)
            .bind(&self.description)
            .bind(&self.tags)
            .execute(conn)
            .await
    }

    pub async fn update(
        &self,
        conn: &mut sqlx::PgConnection,
    ) -> anyhow::Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query(
            "UPDATE links SET url = $1, title = $2, description = $3, tags = $4 WHERE id = $5",
        )
        .bind(&self.url)
        .bind(&self.title)
        .bind(&self.description)
        .bind(&self.tags)
        .bind(&self.id)
        .execute(conn)
        .await
    }

    pub fn print(&self, format: &PrintFormat) -> anyhow::Result<()> {
        match format {
            PrintFormat::Jsonl => {
                serde_json::to_writer(std::io::stdout().lock(), &self)
                    .context("json serialize failed")?;
                print!("\n");
            }
            PrintFormat::Tsv => println!(
                "{}\t{}\t{}\t{}\t{}",
                self.id,
                self.title,
                self.description.as_ref().unwrap_or(&"-".to_string()),
                self.tags.join(","),
                self.url,
            ),
            PrintFormat::Pretty => {
                print!("\x1b[1;38;5;3m");
                print!("{: <4}", self.id);
                print!("\x1b[0m");
                print!("{}\n", self.title);

                if let Some(desc) = &self.description {
                    print!("    ");
                    print!("\x1b[38;5;7m");
                    print!("description: ");
                    print!("\x1b[0m");
                    print!("{}\n", desc);
                }

                print!("    ");
                print!("\x1b[38;5;7m");
                print!("url:         ");
                print!("\x1b[0m");
                print!("\x1b[38;5;4m");
                print!("{}\n", self.url);
                print!("\x1b[0m");

                if !self.tags.is_empty() {
                    print!("    ");
                    print!("\x1b[38;5;7m");
                    print!("tags:        ");
                    print!("\x1b[0m");
                    print!("{}\n", self.tags.join(","));
                }
            }
        }
        Ok(())
    }
}
