use scraper::html;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct News {
    body: String,
    reference_to_photo: String,
    time_string: String,
    title: String,
}

#[derive(Clone, Copy)]
pub enum SortType {
    Date,
    Revalet,
}

impl ToString for SortType {
    fn to_string(&self) -> String {
        match self {
            SortType::Date => "date",
            SortType::Revalet => "relevance",
        }
        .to_string()
    }
}

impl News {
    fn from_nblock(block: String) -> News {
        fn select_title() -> scraper::Selector {
            scraper::Selector::parse("div>h3>a").unwrap()
        }

        fn select_time() -> scraper::Selector {
            scraper::Selector::parse("div>div.ndt").unwrap()
        }
        fn select_body() -> scraper::Selector {
            scraper::Selector::parse("div>p").unwrap()
        }
        fn get_image_src(document: &scraper::Html) -> String {
            let selector = scraper::Selector::parse("a>img").unwrap();
            document
                .select(&selector)
                .last()
                .unwrap()
                .attr("data-src")
                .unwrap()
                .to_string()
        }

        let document = scraper::Html::parse_fragment(&block);
        News {
            title: document
                .select(&select_title())
                .last()
                .unwrap()
                .inner_html(),
            time_string: document.select(&select_time()).last().unwrap().inner_html(),
            reference_to_photo: get_image_src(&document),
            body: document.select(&select_body()).last().unwrap().inner_html(),
        }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn image_src(&self) -> String {
        self.reference_to_photo.clone()
    }

    pub fn time_published(&self) -> String {
        self.time_string.clone()
    }
}

async fn get_serach_news_html(promt: String, sort_type: SortType, page: u32) -> String {
    let url = format!(
        "https://www.gazeta.uz/ru/search?q={}&sort={}",
        promt,
        sort_type.to_string()
    );
    let clinet = reqwest::get(url).await.unwrap();
    clinet.text().await.unwrap()
}

async fn parce_news(html: String) -> Vec<News> {
    let document = scraper::Html::parse_document(&html);
    let cursor =
        scraper::Selector::parse("body>div>div.lenta>div.leftContainer>div>div.nblock").unwrap();
    document
        .select(&cursor)
        .map(|x| News::from_nblock(x.inner_html()))
        .collect()
}

pub async fn search_news(promt: String, sort_type: SortType) -> Vec<News> {
    let mut news = Vec::new();
    for page_number in 1..u32::MAX {
        let mut news_in_iteration =
            parce_news(get_serach_news_html(promt.clone(), sort_type, page_number).await).await;
        if news_in_iteration.is_empty() {
            break;
        }
        news.append(&mut news_in_iteration);
    }
    news
}

#[test]
pub fn test_parsing_news() {
    let html = r#"
        <div class="nblock ">
                        <a href="/ru/2023/09/18/caex/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2023/09/wf2RKB16951062217783_m.jpg" width="180" height="120" alt="В Ташкенте для посетителей CAEx Mebel &amp; Décor 2023 будут организованы 50 бесплатных автобусов" src="https://www.gazeta.uz/media/img/2023/09/wf2RKB16951062217783_m.jpg" data-loaded="true">
                        </a>
                        <div class="nt">
                            <div class="ndt">18 сентября 2023, 21:00</div>
                            <h3>
                                <a href="/ru/2023/09/18/caex/">В&nbsp;Ташкенте для посетителей CAEx Mebel &amp; Décor 2023 будут организованы 50 бесплатных автобусов</a>
                            </h3>
                            <p>Совсем скоро в&nbsp;Ташкенте состоится международная выставка-ярмарка CAEx Mebel &amp; Décor 2023. Для удобства посетителей организованы 50 бесплатных автобусов, на&nbsp;которых можно будет добраться до&nbsp;выставки.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
    "#;

    let target_news = News{ body: "Совсем скоро в&nbsp;Ташкенте состоится международная выставка-ярмарка CAEx Mebel &amp; Décor 2023. Для удобства посетителей организованы 50 бесплатных автобусов, на&nbsp;которых можно будет добраться до&nbsp;выставки.".to_string(), reference_to_photo: "https://www.gazeta.uz/media/img/2023/09/wf2RKB16951062217783_m.jpg".to_string(), time_string: "18 сентября 2023, 21:00".to_string(), title: "В&nbsp;Ташкенте для посетителей CAEx Mebel &amp; Décor 2023 будут организованы 50 бесплатных автобусов".to_string() }
;
    assert_eq!(target_news, News::from_nblock(html.to_string()));
}
