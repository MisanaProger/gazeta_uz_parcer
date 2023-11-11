use std::char::ParseCharError;

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

fn parce_news(html: String) -> Vec<News> {
    let document = scraper::Html::parse_document(&html);
    let cursor =
        scraper::Selector::parse("body>div>div.lenta>div.leftContainer>div.blockSectionNews>div.newsblock-2>div.nblock").unwrap();
    document
        .select(&cursor)
        .map(|x| News::from_nblock(x.inner_html()))
        .collect()
}

pub async fn search_news(promt: String, sort_type: SortType) -> Vec<News> {
    let mut news = Vec::new();
    for page_number in 1..u32::MAX {
        let mut news_in_iteration = search_news_on_page(&promt, sort_type, page_number).await;
        if news_in_iteration.is_empty() {
            break;
        }
        news.append(&mut news_in_iteration);
    }
    news
}

pub async fn search_news_on_page(
    promt: &String,
    sort_type: SortType,
    page_number: u32,
) -> Vec<News> {
    parce_news(get_serach_news_html(promt.clone(), sort_type, page_number).await)
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

#[test]
pub fn test_parsing_list_of_news() {
    let html = r##"<html lang="ru" prefix="article: http://ogp.me/ns/article#"><head><script async="" src="https://mc.yandex.ru/metrika/tag.js"></script><script type="text/javascript" async="" src="https://www.google-analytics.com/analytics.js"></script><script type="text/javascript" async="" src="https://www.googletagmanager.com/gtag/js?id=G-JQ1MW042RB&amp;l=dataLayer&amp;cx=c"></script><script src="https://connect.facebook.net/signals/config/440076653520103?v=2.9.138&amp;r=stable&amp;domain=www.gazeta.uz" async=""></script><script src="https://connect.facebook.net/en_US/sdk.js?hash=302cbd0e3577ebf7c5bb9aa83174397f" async="" crossorigin="anonymous"></script><script async="" src="https://connect.facebook.net/en_US/fbevents.js"></script><script type="text/javascript" async="" src="https://www.gstatic.com/recaptcha/releases/fGZmEzpfeSeqDJiApS_XZ4Y2/recaptcha__ru.js" crossorigin="anonymous" integrity="sha384-w5tKS4dWm1I9ArlRHs6tlR2xhb7YwVwnGz5IAuSB1oxtbjeaNLHXA+kbIAuK/rDg"></script><script src="https://apis.google.com/_/scs/abc-static/_/js/k=gapi.lb.ru.SdJ4lx3vEGc.O/m=auth2/rt=j/sv=1/d=1/ed=1/rs=AHpOoo90e1AagTP5OTGtZlZ1j9Pz6wSe0w/cb=gapi.loaded_0?le=scs" async=""></script>
    <title>Результаты поиска по фразе «пр» – Газета.uz</title>
	<meta http-equiv="content-type" content="text/html;charset=utf-8">
    <meta content="initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no" name="viewport">
    <meta name="apple-itunes-app" content="app-id=550064053">
    <meta name="theme-color" content="#3b83f2">
                <link rel="manifest" href="https://www.gazeta.uz/manifest.json?r=v230925">
                <link rel="canonical" href="https://www.gazeta.uz/ru/search?q=пр">
        <link rel="alternate" hreflang="ru" href="https://www.gazeta.uz/ru/search"> 
    <link rel="alternate" hreflang="uz-Latn" href="https://www.gazeta.uz/oz/search"> 
    <link rel="alternate" hreflang="uz-Cyrl" href="https://www.gazeta.uz/uz/search"> 
	<meta property="og:site_name" content="Газета.uz">
	<meta property="fb:app_id" content="501898193157496">

    
    
    <meta name="description" content="Результаты поиска по фразе 'пр'">
    <meta name="og:description" content="Результаты поиска по фразе 'пр'">
                <link rel="apple-touch-icon" sizes="180x180" href="/i/icon/apple-touch-icon.png">
    <link rel="icon" href="/i/icon/favicon.svg">
    <link rel="icon" type="image/png" sizes="32x32" href="/i/icon/favicon-32x32.png">
    <link rel="icon" type="image/png" sizes="16x16" href="/i/icon/favicon-16x16.png">
    <link rel="mask-icon" href="/i/icon/safari-pinned-tab.svg" color="#5bbad5">
    <meta name="msapplication-TileColor" content="#3b83f2">
    <link rel="alternate" href="https://www.gazeta.uz/ru/rss/" type="application/rss+xml" title="Новости Узбекистана – Газета.uz">
        <link rel="preconnect" href="https://oa.afishamedia.net" crossorigin="">
    <link rel="preconnect" href="https://yandex.ru" crossorigin="">
    <link rel="preconnect" href="https://yastatic.net" crossorigin="">
    <link rel="preconnect" href="https://mc.yandex.ru" crossorigin="">
    <link rel="preconnect" href="https://connect.facebook.net" crossorigin="">
    <link rel="preconnect" href="https://www.googletagmanager.com" crossorigin="">
    <link rel="preconnect" href="https://www.gstatic.com" crossorigin="">
    <link rel="preconnect" href="https://www.google.com" crossorigin="">
    <link rel="preconnect" href="https://apis.google.com" crossorigin="">
    <link rel="preconnect" href="https://cdnjs.cloudflare.com" crossorigin="">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="">
    <link rel="preload" href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,400;0,500;0,700;1,400&amp;display=swap" as="style">
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,400;0,500;0,700;1,400&amp;display=swap" media="all" onload="this.media='all'">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.13.0/css/all.min.css" integrity="sha512-L7MWcK7FNPcwNqnLdZq86lTHYLdQqZaz5YcAgE+5cnGmlw8JT03QB2+oxL100UeB6RlzZLUxCGSS4/++mNZdxw==" crossorigin="anonymous" referrerpolicy="no-referrer" media="all" onload="this.media='all'">
    <link rel="stylesheet" href="/css/comments.css?r=v230929" media="screen">
    <link rel="stylesheet" href="/css/auth.css?r=v230929" media="screen">
    <link type="text/css" href="/css/main.css?r=v230929" rel="stylesheet" media="screen">
    <script async="" src="https://www.googletagmanager.com/gtm.js?id=GTM-K84J9WF"></script><script type="text/javascript" src="/js/jquery.min.js?r=v230925"></script>
    <script type="text/javascript" src="/js/lazy.min.js?r=v230925"></script>
    <script type="text/javascript" src="/js/init.js?r=v230925"></script>
    <script type="text/javascript" src="/js/search.js?r=v230925"></script>
	<script><!--// <![CDATA[
	var OA_source = 'Результаты поиска по фразе &laquo;пр&raquo;';
	// ]]> --></script>
	<script>(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
	new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
	j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
	'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
	})(window,document,'script','dataLayer','GTM-K84J9WF');</script>
	    <script>
    window.dataLayer = window.dataLayer || [];
    window.dataLayer.push ({
        'lang': 'ru',
            });
    </script>
    <script>window.yaContextCb = window.yaContextCb || []</script>
    <script src="https://yandex.ru/ads/system/context.js" async=""></script>
<link rel="preconnect" href="https://yastatic.net/"><link rel="preconnect" href="https://avatars.mds.yandex.net/"><link rel="preconnect" href="https://mc.yandex.ru/"><link rel="preconnect" href="https://ads.adfox.ru"><script async="" crossorigin="anonymous" src="https://yastatic.net/safeframe-bundles/0.83/host.js"></script><link rel="preload" href="https://yastatic.net/s3/home/fonts/ys/3/text-variable-full.woff2" type="font/woff2" as="font" crossorigin="anonymous"><style id="ysTextCssRule">@font-face {
        font-family: "YS Text Variable";
        src: url("https://yastatic.net/s3/home/fonts/ys/3/text-variable-full.woff2") format("woff2");
        font-weight: 400 700;
        font-display: optional;
    }</style><style type="text/css" data-fbcssmodules="css:fb.css.base css:fb.css.dialog css:fb.css.iframewidget css:fb.css.customer_chat_plugin_iframe">.fb_hidden{position:absolute;top:-10000px;z-index:10001}.fb_reposition{overflow:hidden;position:relative}.fb_invisible{display:none}.fb_reset{background:none;border:0;border-spacing:0;color:#000;cursor:auto;direction:ltr;font-family:'lucida grande', tahoma, verdana, arial, sans-serif;font-size:11px;font-style:normal;font-variant:normal;font-weight:normal;letter-spacing:normal;line-height:1;margin:0;overflow:visible;padding:0;text-align:left;text-decoration:none;text-indent:0;text-shadow:none;text-transform:none;visibility:visible;white-space:normal;word-spacing:normal}.fb_reset>div{overflow:hidden}@keyframes fb_transform{from{opacity:0;transform:scale(.95)}to{opacity:1;transform:scale(1)}}.fb_animate{animation:fb_transform .3s forwards}
.fb_hidden{position:absolute;top:-10000px;z-index:10001}.fb_reposition{overflow:hidden;position:relative}.fb_invisible{display:none}.fb_reset{background:none;border:0;border-spacing:0;color:#000;cursor:auto;direction:ltr;font-family:'lucida grande', tahoma, verdana, arial, sans-serif;font-size:11px;font-style:normal;font-variant:normal;font-weight:normal;letter-spacing:normal;line-height:1;margin:0;overflow:visible;padding:0;text-align:left;text-decoration:none;text-indent:0;text-shadow:none;text-transform:none;visibility:visible;white-space:normal;word-spacing:normal}.fb_reset>div{overflow:hidden}@keyframes fb_transform{from{opacity:0;transform:scale(.95)}to{opacity:1;transform:scale(1)}}.fb_animate{animation:fb_transform .3s forwards}
.fb_dialog{background:rgba(82, 82, 82, .7);position:absolute;top:-10000px;z-index:10001}.fb_dialog_advanced{border-radius:8px;padding:10px}.fb_dialog_content{background:#fff;color:#373737}.fb_dialog_close_icon{background:url(https://connect.facebook.net/rsrc.php/v3/yq/r/IE9JII6Z1Ys.png) no-repeat scroll 0 0 transparent;cursor:pointer;display:block;height:15px;position:absolute;right:18px;top:17px;width:15px}.fb_dialog_mobile .fb_dialog_close_icon{left:5px;right:auto;top:5px}.fb_dialog_padding{background-color:transparent;position:absolute;width:1px;z-index:-1}.fb_dialog_close_icon:hover{background:url(https://connect.facebook.net/rsrc.php/v3/yq/r/IE9JII6Z1Ys.png) no-repeat scroll 0 -15px transparent}.fb_dialog_close_icon:active{background:url(https://connect.facebook.net/rsrc.php/v3/yq/r/IE9JII6Z1Ys.png) no-repeat scroll 0 -30px transparent}.fb_dialog_iframe{line-height:0}.fb_dialog_content .dialog_title{background:#6d84b4;border:1px solid #365899;color:#fff;font-size:14px;font-weight:bold;margin:0}.fb_dialog_content .dialog_title>span{background:url(https://connect.facebook.net/rsrc.php/v3/yd/r/Cou7n-nqK52.gif) no-repeat 5px 50%;float:left;padding:5px 0 7px 26px}body.fb_hidden{height:100%;left:0;margin:0;overflow:visible;position:absolute;top:-10000px;transform:none;width:100%}.fb_dialog.fb_dialog_mobile.loading{background:url(https://connect.facebook.net/rsrc.php/v3/ya/r/3rhSv5V8j3o.gif) white no-repeat 50% 50%;min-height:100%;min-width:100%;overflow:hidden;position:absolute;top:0;z-index:10001}.fb_dialog.fb_dialog_mobile.loading.centered{background:none;height:auto;min-height:initial;min-width:initial;width:auto}.fb_dialog.fb_dialog_mobile.loading.centered #fb_dialog_loader_spinner{width:100%}.fb_dialog.fb_dialog_mobile.loading.centered .fb_dialog_content{background:none}.loading.centered #fb_dialog_loader_close{clear:both;color:#fff;display:block;font-size:18px;padding-top:20px}#fb-root #fb_dialog_ipad_overlay{background:rgba(0, 0, 0, .4);bottom:0;left:0;min-height:100%;position:absolute;right:0;top:0;width:100%;z-index:10000}#fb-root #fb_dialog_ipad_overlay.hidden{display:none}.fb_dialog.fb_dialog_mobile.loading iframe{visibility:hidden}.fb_dialog_mobile .fb_dialog_iframe{position:sticky;top:0}.fb_dialog_content .dialog_header{background:linear-gradient(from(#738aba), to(#2c4987));border-bottom:1px solid;border-color:#043b87;box-shadow:white 0 1px 1px -1px inset;color:#fff;font:bold 14px Helvetica, sans-serif;text-overflow:ellipsis;text-shadow:rgba(0, 30, 84, .296875) 0 -1px 0;vertical-align:middle;white-space:nowrap}.fb_dialog_content .dialog_header table{height:43px;width:100%}.fb_dialog_content .dialog_header td.header_left{font-size:12px;padding-left:5px;vertical-align:middle;width:60px}.fb_dialog_content .dialog_header td.header_right{font-size:12px;padding-right:5px;vertical-align:middle;width:60px}.fb_dialog_content .touchable_button{background:linear-gradient(from(#4267B2), to(#2a4887));background-clip:padding-box;border:1px solid #29487d;border-radius:3px;display:inline-block;line-height:18px;margin-top:3px;max-width:85px;padding:4px 12px;position:relative}.fb_dialog_content .dialog_header .touchable_button input{background:none;border:none;color:#fff;font:bold 12px Helvetica, sans-serif;margin:2px -12px;padding:2px 6px 3px 6px;text-shadow:rgba(0, 30, 84, .296875) 0 -1px 0}.fb_dialog_content .dialog_header .header_center{color:#fff;font-size:16px;font-weight:bold;line-height:18px;text-align:center;vertical-align:middle}.fb_dialog_content .dialog_content{background:url(https://connect.facebook.net/rsrc.php/v3/y9/r/jKEcVPZFk-2.gif) no-repeat 50% 50%;border:1px solid #4a4a4a;border-bottom:0;border-top:0;height:150px}.fb_dialog_content .dialog_footer{background:#f5f6f7;border:1px solid #4a4a4a;border-top-color:#ccc;height:40px}#fb_dialog_loader_close{float:left}.fb_dialog.fb_dialog_mobile .fb_dialog_close_icon{visibility:hidden}#fb_dialog_loader_spinner{animation:rotateSpinner 1.2s linear infinite;background-color:transparent;background-image:url(https://connect.facebook.net/rsrc.php/v3/yD/r/t-wz8gw1xG1.png);background-position:50% 50%;background-repeat:no-repeat;height:24px;width:24px}@keyframes rotateSpinner{0%{transform:rotate(0deg)}100%{transform:rotate(360deg)}}
.fb_iframe_widget{display:inline-block;position:relative}.fb_iframe_widget span{display:inline-block;position:relative;text-align:justify}.fb_iframe_widget iframe{position:absolute}.fb_iframe_widget_fluid_desktop,.fb_iframe_widget_fluid_desktop span,.fb_iframe_widget_fluid_desktop iframe{max-width:100%}.fb_iframe_widget_fluid_desktop iframe{min-width:220px;position:relative}.fb_iframe_widget_lift{z-index:1}.fb_iframe_widget_fluid{display:inline}.fb_iframe_widget_fluid span{width:100%}
.fb_mpn_mobile_landing_page_slide_out{animation-duration:200ms;animation-name:fb_mpn_landing_page_slide_out;transition-timing-function:ease-in}.fb_mpn_mobile_landing_page_slide_out_from_left{animation-duration:200ms;animation-name:fb_mpn_landing_page_slide_out_from_left;transition-timing-function:ease-in}.fb_mpn_mobile_landing_page_slide_up{animation-duration:500ms;animation-name:fb_mpn_landing_page_slide_up;transition-timing-function:ease-in}.fb_mpn_mobile_bounce_in{animation-duration:300ms;animation-name:fb_mpn_bounce_in;transition-timing-function:ease-in}.fb_mpn_mobile_bounce_out{animation-duration:300ms;animation-name:fb_mpn_bounce_out;transition-timing-function:ease-in}.fb_mpn_mobile_bounce_out_v2{animation-duration:300ms;animation-name:fb_mpn_fade_out;transition-timing-function:ease-in}.fb_customer_chat_bounce_in_v2{animation-duration:300ms;animation-name:fb_bounce_in_v2;transition-timing-function:ease-in}.fb_customer_chat_bounce_in_from_left{animation-duration:300ms;animation-name:fb_bounce_in_from_left;transition-timing-function:ease-in}.fb_customer_chat_bounce_out_v2{animation-duration:300ms;animation-name:fb_bounce_out_v2;transition-timing-function:ease-in}.fb_customer_chat_bounce_out_from_left{animation-duration:300ms;animation-name:fb_bounce_out_from_left;transition-timing-function:ease-in}.fb_invisible_flow{display:inherit;height:0;overflow-x:hidden;width:0}@keyframes fb_mpn_landing_page_slide_out{0%{margin:0 12px;width:100% - 24px}60%{border-radius:18px}100%{border-radius:50%;margin:0 24px;width:60px}}@keyframes fb_mpn_landing_page_slide_out_from_left{0%{left:12px;width:100% - 24px}60%{border-radius:18px}100%{border-radius:50%;left:12px;width:60px}}@keyframes fb_mpn_landing_page_slide_up{0%{bottom:0;opacity:0}100%{bottom:24px;opacity:1}}@keyframes fb_mpn_bounce_in{0%{opacity:.5;top:100%}100%{opacity:1;top:0}}@keyframes fb_mpn_fade_out{0%{bottom:30px;opacity:1}100%{bottom:0;opacity:0}}@keyframes fb_mpn_bounce_out{0%{opacity:1;top:0}100%{opacity:.5;top:100%}}@keyframes fb_bounce_in_v2{0%{opacity:0;transform:scale(0, 0);transform-origin:bottom right}50%{transform:scale(1.03, 1.03);transform-origin:bottom right}100%{opacity:1;transform:scale(1, 1);transform-origin:bottom right}}@keyframes fb_bounce_in_from_left{0%{opacity:0;transform:scale(0, 0);transform-origin:bottom left}50%{transform:scale(1.03, 1.03);transform-origin:bottom left}100%{opacity:1;transform:scale(1, 1);transform-origin:bottom left}}@keyframes fb_bounce_out_v2{0%{opacity:1;transform:scale(1, 1);transform-origin:bottom right}100%{opacity:0;transform:scale(0, 0);transform-origin:bottom right}}@keyframes fb_bounce_out_from_left{0%{opacity:1;transform:scale(1, 1);transform-origin:bottom left}100%{opacity:0;transform:scale(0, 0);transform-origin:bottom left}}@keyframes slideInFromBottom{0%{opacity:.1;transform:translateY(100%)}100%{opacity:1;transform:translateY(0)}}@keyframes slideInFromBottomDelay{0%{opacity:0;transform:translateY(100%)}97%{opacity:0;transform:translateY(100%)}100%{opacity:1;transform:translateY(0)}}</style><meta http-equiv="origin-trial" content="AymqwRC7u88Y4JPvfIF2F37QKylC04248hLCdJAsh8xgOfe/dVJPV3XS3wLFca1ZMVOtnBfVjaCMTVudWM//5g4AAAB7eyJvcmlnaW4iOiJodHRwczovL3d3dy5nb29nbGV0YWdtYW5hZ2VyLmNvbTo0NDMiLCJmZWF0dXJlIjoiUHJpdmFjeVNhbmRib3hBZHNBUElzIiwiZXhwaXJ5IjoxNjk1MTY3OTk5LCJpc1RoaXJkUGFydHkiOnRydWV9"><meta http-equiv="origin-trial" content="AymqwRC7u88Y4JPvfIF2F37QKylC04248hLCdJAsh8xgOfe/dVJPV3XS3wLFca1ZMVOtnBfVjaCMTVudWM//5g4AAAB7eyJvcmlnaW4iOiJodHRwczovL3d3dy5nb29nbGV0YWdtYW5hZ2VyLmNvbTo0NDMiLCJmZWF0dXJlIjoiUHJpdmFjeVNhbmRib3hBZHNBUElzIiwiZXhwaXJ5IjoxNjk1MTY3OTk5LCJpc1RoaXJkUGFydHkiOnRydWV9"><meta http-equiv="origin-trial" content="A+xK4jmZTgh1KBVry/UZKUE3h6Dr9HPPioFS4KNCzify+KEoOii7z/goKS2zgbAOwhpZ1GZllpdz7XviivJM9gcAAACFeyJvcmlnaW4iOiJodHRwczovL3d3dy5nb29nbGV0YWdtYW5hZ2VyLmNvbTo0NDMiLCJmZWF0dXJlIjoiQXR0cmlidXRpb25SZXBvcnRpbmdDcm9zc0FwcFdlYiIsImV4cGlyeSI6MTcwNzI2Mzk5OSwiaXNUaGlyZFBhcnR5Ijp0cnVlfQ=="><script attributionsrc="" type="text/javascript" async="" src="https://www.google.com/pagead/1p-conversion/615909469/?random=1699711711336&amp;cv=11&amp;fst=1699711711336&amp;bg=ffffff&amp;guid=ON&amp;async=1&amp;gtm=45He3b81v813181448&amp;gcd=11l1l1l1l1&amp;dma=0&amp;u_w=1366&amp;u_h=768&amp;url=https%3A%2F%2Fwww.gazeta.uz%2Fru%2Fsearch%2F%3Fq%3D%25D0%25BF%25D1%2580%26sort%3Drelevance%26page%3D1&amp;label=GzrTCLrG3NYBEN2Q2KUC&amp;hn=www.google.com&amp;frm=0&amp;tiba=%D0%A0%D0%B5%D0%B7%D1%83%D0%BB%D1%8C%D1%82%D0%B0%D1%82%D1%8B%20%D0%BF%D0%BE%D0%B8%D1%81%D0%BA%D0%B0%20%D0%BF%D0%BE%20%D1%84%D1%80%D0%B0%D0%B7%D0%B5%20%C2%AB%D0%BF%D1%80%C2%BB%20%E2%80%93%20%D0%93%D0%B0%D0%B7%D0%B5%D1%82%D0%B0.uz&amp;value=0&amp;bttype=purchase&amp;auid=488280427.1699630684&amp;gcp=1&amp;sscte=1&amp;ct_cookie_present=1&amp;rfmt=3&amp;fmt=4"></script><meta http-equiv="origin-trial" content="AymqwRC7u88Y4JPvfIF2F37QKylC04248hLCdJAsh8xgOfe/dVJPV3XS3wLFca1ZMVOtnBfVjaCMTVudWM//5g4AAAB7eyJvcmlnaW4iOiJodHRwczovL3d3dy5nb29nbGV0YWdtYW5hZ2VyLmNvbTo0NDMiLCJmZWF0dXJlIjoiUHJpdmFjeVNhbmRib3hBZHNBUElzIiwiZXhwaXJ5IjoxNjk1MTY3OTk5LCJpc1RoaXJkUGFydHkiOnRydWV9"></head>
<body data-recaptcha-loaded="1">
<noscript><iframe src="https://www.googletagmanager.com/ns.html?id=GTM-K84J9WF"
height="0" width="0" style="display:none;visibility:hidden"></iframe></noscript>
<div id="app" data-auth-provider="" data-auth-username="" data-auth-image="">
        <div class="nav">
        <div class="nav__container">
            <div class="nav__container-logo">
				<a href="/ru/"><img src="/i/gazeta_logo.png" alt="Газета.uz" title="Газета.uz" width="120"></a>
            </div>
            <ul class="nav__container-items">
                                                    <li class="f"><a href="/ru/column/">Колонки и интервью</a></li>
                                    <li><a href="/ru/society/">Общество</a></li>
                                    <li><a href="/ru/politics/">Политика</a></li>
                                    <li><a href="/ru/economy/">Экономика</a></li>
                                    <li><a href="/ru/sport/">Спорт</a></li>
                                    <li><a href="/ru/world/">Мир</a></li>
                                                                                                                </ul>
            <div class="nav__container-right">
                <div class="nav__container-lang">
                    <a href="https://www.gazeta.uz/ru/" class="ru current">
                    <span>
                        Рус                    </span>
                    </a>
                    <a href="https://www.gazeta.uz/oz/" class="oz">
                    <span>
                        O‘zb                    </span>
                    </a>
                    <a href="https://www.gazeta.uz/uz/" class="uz">
                    <span>
                        Ўзб                    </span>
                    </a>                    
                </div>
                <div class="nav__container-search nav__container-search-active">
                    <div class="nav__container-search-go"><i class="fas fa-search" aria-hidden="true"></i></div>
                    <div class="nav__container-search-form">
                        <form action="/ru/search/" method="get" name="search">
                            <input name="q" value="" type="text" class="searchinput" maxlength="32">
                        </form>
                    </div>
                    <div class="nav__container-search-close"><i class="fas fa-times" aria-hidden="true"></i></div>
                </div>
                <div class="nav__container-user-menu">
                    <a class="btn-user-menu login-modal-trigger" href="#" title="Войти"><i class="fas fa-user-circle"></i></a>
                    <a class="btn-user-menu user-menu-dropdown-trigger hidden" href="#" title=""><img src="/i/no_avatar.png" class="user-image user-image__updated" alt=""></a>
                    <ul class="user-menu">
                        <li><a href="https://www.gazeta.uz/ru/users/profile/"><i class="fas fa-fw fa-user"></i> Профиль</a></li>
                        <li><a href="#" class="btn-auth-logout"><i class="fas fa-fw fa-sign-out-alt"></i> Выйти</a></li>
                    </ul>
                </div>
            </div>
        </div>
    </div>
    <div class="clear"></div>
    <div class="lenta">
                                                 
							<div class="lb-970">
					<iframe id="add6ad5c" name="add6ad5c" loading="lazy" src="//oa.afishamedia.net/www/delivery/afr.php?zoneid=79&amp;source=search&amp;cb=1914125448" frameborder="0" scrolling="no" width="970" height="250"><a href='//oa.afishamedia.net/www/delivery/ck.php?n=aeae4f26&amp;cb=1914125448' target='_blank'><img src='//oa.afishamedia.net/www/delivery/avw.php?zoneid=79&amp;source=search&amp;cb=1914125448&amp;n=aeae4f26' loading='lazy' border='0' alt='' /></a></iframe>
				</div>
                    
                <div class="head-container">
            <ul class="head-container-items">
                <li><a href="/ru/">Главная</a></li>
                <li><a href="/ru/list/news/">Новости</a></li>
                <li><a href="/ru/list/articles/">Статьи</a></li>
                <li><a href="/ru/list/reporting/">Репортажи</a></li>
                <li><a href="/ru/list/media/">Медиа</a></li>
            </ul>

            <span class="head-container-date">
                11 ноября, суббота            </span>

            <span class="head-container-links">
                								<a href="https://www.afisha.uz/" target="_blank">Afisha</a>
				<a href="https://www.spot.uz/" target="_blank">Spot</a>
				<a href="https://zira.uz/" target="_blank">Zira</a>
				<a href="https://pogoda.uz/" target="_blank">Погода</a>
				<a href="https://docs.google.com/forms/d/e/1FAIpQLSdhU1dTDFfckVBBCyZP7XN1Dgdy9N1Qy5mo6Su0KfXMuRzJlA/viewform" target="_blank">Вакансии</a>
                                <div class="ish-uz-link">
                    <a href="https://ish.uz/ru" target="_blank"><img src="/i/ish-label.svg" alt="Ish.uz"><span>Ish.uz</span></a>
                </div>
            </span>
            <div class="clear"></div>
        </div>

                
<div class="leftContainer">
            <select name="sort" id="search_sort" class="search-sort-select">
            <option value="" disabled="">Сортировка</option>
                            <option value="relevance" selected="">По релевантности</option>
                            <option value="date">По дате</option>
                    </select>
    	<h2>
		Результаты поиска по фразе «пр»	</h2>
    <br>

	<div class="blockSectionNews">
                    <div class="newsblock-2">
                                    
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
                                    <div class="nblock ">
                        <a href="/ru/2023/08/29/akfa-build/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2023/08/D3BtHZ16932876015477_m.jpg" width="180" height="120" alt="AKFA Build расширил географию экспорта в сторону Казахстана">
                        </a>
                        <div class="nt">
                            <div class="ndt">29 августа 2023, 17:00</div>
                            <h3>
                                <a href="/ru/2023/08/29/akfa-build/">AKFA Build расширил географию экспорта в&nbsp;сторону Казахстана</a>
                            </h3>
                            <p>Компания AKFA Build открыла первый мультибрендовый шоурум в&nbsp;Шымкенте. На&nbsp;церемонии открытия участникам рассказали о&nbsp;линейке продукции компании, также производственном потенциале и&nbsp;планах на&nbsp;будущее.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2023/07/12/umed/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2023/07/tOO6tO16891475889934_m.jpg" width="180" height="120" alt="Дипломатическая академия при УМЭД объявила о наборе на магистратуру">
                        </a>
                        <div class="nt">
                            <div class="ndt">12 июля 2023, 15:00</div>
                            <h3>
                                <a href="/ru/2023/07/12/umed/">Дипломатическая академия при УМЭД объявила о&nbsp;наборе на&nbsp;магистратуру</a>
                            </h3>
                            <p>Стартовал приём документов на&nbsp;магистратуру в&nbsp;Дипломатическую академию при Университете мировой экономики и&nbsp;дипломатии. Документы необходимо подать онлайн. Форма обучения&nbsp;— очная.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2023/06/21/imperial-club-city/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2023/06/emigo416869009428750_m.jpg" width="180" height="120" alt="На что обратить внимание при выборе ультра-премиального ЖК">
                        </a>
                        <div class="nt">
                            <div class="ndt">21 июня 2023, 13:00</div>
                            <h3>
                                <a href="/ru/2023/06/21/imperial-club-city/">На&nbsp;что обратить внимание при выборе ультра-премиального&nbsp;ЖК</a>
                            </h3>
                            <p>В&nbsp;Imperial Club City назвали основные характеристики элитной жилой недвижимости в&nbsp;центре Ташкента.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2023/09/10/hot-water/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2023/02/SKHXtW16757713053020_m.jpg" width="180" height="120" alt="Горячую воду отключат в ряде районов Ташкента 11−15 сентября" src="https://www.gazeta.uz/media/img/2023/02/SKHXtW16757713053020_m.jpg" data-loaded="true">
                        </a>
                        <div class="nt">
                            <div class="ndt">10 сентября 2023, 07:16&nbsp;<span class="ico-comm"><i class="fas fa-comment" aria-hidden="true"></i><span>6</span></span></div>
                            <h3>
                                <a href="/ru/2023/09/10/hot-water/">Горячую воду отключат в&nbsp;ряде районов Ташкента 11−15&nbsp;сентября</a>
                            </h3>
                            <p>В&nbsp;связи с&nbsp;плановыми ремонтными работами на&nbsp;Ташкентской ТЭЦ с&nbsp;11 по&nbsp;15&nbsp;сентября в&nbsp;ряде районов столицы будет отсутствовать горячее водоснабжение. Список адресов.</p>
                        </div>
                        <div class="clear"></div>
                    </div><div class="nblock ">
                        <a href="/ru/2023/09/22/sharq-building/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2023/09/SG3CRf16953645805618_m.jpg" width="180" height="120" alt="«Остов коллективной памяти ташкентцев». Борис Чухович — о высотном здании «Шарк» (УзА)">
                        </a>
                        <div class="nt">
                            <div class="ndt">22 сентября 2023, 19:02&nbsp;<span class="ico-comm"><i class="fas fa-comment" aria-hidden="true"></i><span>4</span></span></div>
                            <h3>
                                <a href="/ru/2023/09/22/sharq-building/">«Остов коллективной памяти ташкентцев». Борис Чухович&nbsp;— о&nbsp;высотном здании «Шарк» (УзА)</a>
                            </h3>
                            <p>16-этажное здание издательства «Шарк» архитектора Ричарда Блезэ, несмотря на&nbsp;неправильную реконструкцию 10 лет назад, остаётся иконой Ташкента, считает историк архитектуры Борис Чухович. В&nbsp;колонке для «Газеты.uz» он&nbsp;пишет, почему здания «Шарк» (включая горизонтальный корпус) должны быть сохранены.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2023/06/14/umed/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2023/06/qEqLrS16866627015894_m.jpg" width="180" height="120" alt="УМЭД приглашает на День открытых дверей">
                        </a>
                        <div class="nt">
                            <div class="ndt">14 июня 2023, 09:00</div>
                            <h3>
                                <a href="/ru/2023/06/14/umed/">УМЭД приглашает на&nbsp;День открытых дверей</a>
                            </h3>
                            <p>15&nbsp;июня в&nbsp;Университете мировой экономики и&nbsp;дипломатии состоится День открытых дверей. Абитуриенты смогут ознакомиться с&nbsp;условиями поступления, а&nbsp;также пообщаться с&nbsp;преподавателями и&nbsp;студентами.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2022/12/17/goodzone/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2022/12/yligsW16711777662271_m.jpg" width="180" height="120" alt="GOODZONE объявил новогоднюю акцию">
                        </a>
                        <div class="nt">
                            <div class="ndt">17 декабря 2022, 17:00</div>
                            <h3>
                                <a href="/ru/2022/12/17/goodzone/">GOODZONE объявил новогоднюю акцию</a>
                            </h3>
                            <p>В&nbsp;сети бытовой техники и&nbsp;электроники GOODZONE действуют скидки до&nbsp;50% на&nbsp;все категории товаров.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2022/12/05/seoul-mun/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2022/12/wAaKGO16700669205269_m.jpg" width="180" height="120" alt="Seoul Mun: помещения от 26 до 452 кв. м для разных видов бизнеса">
                        </a>
                        <div class="nt">
                            <div class="ndt">5 декабря 2022, 09:00</div>
                            <h3>
                                <a href="/ru/2022/12/05/seoul-mun/">Seoul Mun: помещения от&nbsp;26 до&nbsp;452 кв. м&nbsp;для разных видов бизнеса</a>
                            </h3>
                            <p>Оптимальные планировочные решения коммерческих помещений Seoul Mun и&nbsp;удобная транспортная развязка помогут в&nbsp;ведении разных видов бизнеса.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2022/12/27/mercedes-benz/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2022/12/vRGesw16721216806465_m.jpg" width="180" height="120" alt="В Ташкенте откроется новый шоурум Mercedes-Benz">
                        </a>
                        <div class="nt">
                            <div class="ndt">27 декабря 2022, 09:00</div>
                            <h3>
                                <a href="/ru/2022/12/27/mercedes-benz/">В&nbsp;Ташкенте откроется новый шоурум Mercedes-Benz</a>
                            </h3>
                            <p>Теперь автомобили немецкого премиального бренда Mercedes-Benz станут ещё ближе для тех, кто хочет воспользоваться всеми преимуществами, которые даёт обращение к&nbsp;официальному дилеру.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2023/02/21/universe-group/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2023/02/O3VJcB16768975158927_m.jpg" width="180" height="120" alt="Universe Group дарит возможность начать обучение в престижном вузе в 2023 году">
                        </a>
                        <div class="nt">
                            <div class="ndt">21 февраля 2023, 13:00</div>
                            <h3>
                                <a href="/ru/2023/02/21/universe-group/">Universe Group дарит возможность начать обучение в&nbsp;престижном вузе в&nbsp;2023 году</a>
                            </h3>
                            <p>Компания Universe Group рассказала, как консалтинговое агентство может помочь в&nbsp;поступлении в&nbsp;зарубежный вуз, чтобы начать обучение уже в&nbsp;2023 году.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2022/12/06/novastroy/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2022/12/A5NNeR16703066504954_m.jpg" width="180" height="120" alt="Novastroy объявил о скидках и праздничных предложениях">
                        </a>
                        <div class="nt">
                            <div class="ndt">6 декабря 2022, 13:00</div>
                            <h3>
                                <a href="/ru/2022/12/06/novastroy/">Novastroy объявил о&nbsp;скидках и&nbsp;праздничных предложениях</a>
                            </h3>
                            <p>В&nbsp;компании Novastroy действуют выгодные акции в&nbsp;течение всего декабря на&nbsp;офисные помещения в&nbsp;жилом комплексе Parkwood, mixed-use комплексе Livingood и&nbsp;бизнес-центре ONYX.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2018/03/28/hot-water/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2013/07/3cyzWE13749948672692_m.jpg" width="180" height="120" alt="График отключений горячей воды в Ташкенте">
                        </a>
                        <div class="nt">
                            <div class="ndt">28 марта 2018, 10:35&nbsp;<span class="ico-comm"><i class="fas fa-comment" aria-hidden="true"></i><span>3</span></span></div>
                            <h3>
                                <a href="/ru/2018/03/28/hot-water/">График отключений горячей воды в&nbsp;Ташкенте</a>
                            </h3>
                            <p>Плановые весенние отключения горячей воды в&nbsp;Ташкенте продлятся с&nbsp;9&nbsp;апреля по&nbsp;18&nbsp;мая.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2011/08/26/uztelecom/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2011/08/2205_m.jpg" width="180" height="120" alt="«Узбектелеком» открыл новые офисы продаж">
                        </a>
                        <div class="nt">
                            <div class="ndt">26 августа 2011, 17:53</div>
                            <h3>
                                <a href="/ru/2011/08/26/uztelecom/">«Узбектелеком» открыл новые офисы продаж</a>
                            </h3>
                            <p>26 августа в Ташкенте и областях открыто еще 12 офисов продаж услуг компании «Узбектелеком», организованных по принципу «единого окна».</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2022/08/29/meteo/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2021/06/75QW5U16230526620340_m.jpg" width="180" height="120" alt="Праздничные дни будут умеренно жаркими">
                        </a>
                        <div class="nt">
                            <div class="ndt">29 августа 2022, 11:52</div>
                            <h3>
                                <a href="/ru/2022/08/29/meteo/">Праздничные дни будут умеренно жаркими</a>
                            </h3>
                            <p>К&nbsp;началу праздничных выходных по&nbsp;Узбекистану станет немного жарче&nbsp;— до +37, в&nbsp;ночные и&nbsp;утренние часы по-прежнему будет прохладно. Осадков не&nbsp;ожидается.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2019/05/20/chinaexpo/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2019/05/S4upu415583282299217_m.jpg" width="180" height="120" alt="Выставка китайских товаров и услуг пройдет в Ташкенте">
                        </a>
                        <div class="nt">
                            <div class="ndt">20 мая 2019, 12:00</div>
                            <h3>
                                <a href="/ru/2019/05/20/chinaexpo/">Выставка китайских товаров и&nbsp;услуг пройдет в&nbsp;Ташкенте</a>
                            </h3>
                            <p>В «Узэкспоцентре» будет проходить китайская выставка строительной и&nbsp;сельхозтехники, автозапчастей, бытовой техники и&nbsp;товаров для дома.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2018/08/24/stamary-life/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2018/08/huevNa15350886252322_m.jpg" width="180" height="120" alt="Stamary Life поможет абитуриентам поступить в ведущие вузы России">
                        </a>
                        <div class="nt">
                            <div class="ndt">24 августа 2018, 13:00</div>
                            <h3>
                                <a href="/ru/2018/08/24/stamary-life/">Stamary Life поможет абитуриентам поступить в&nbsp;ведущие вузы России</a>
                            </h3>
                            <p>Компания Stamary Life ознакомит с&nbsp;презентациями передовых вузов России и&nbsp;индивидуально проконсультирует абитуриентов.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2020/08/17/new-millennium/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2020/08/JYhK9Q15976373509757_m.jpg" width="180" height="120" alt="New Millennium Concept Store запустил распродажу летней коллекции">
                        </a>
                        <div class="nt">
                            <div class="ndt">17 августа 2020, 13:00</div>
                            <h3>
                                <a href="/ru/2020/08/17/new-millennium/">New Millennium Concept Store запустил распродажу летней коллекции</a>
                            </h3>
                            <p>С&nbsp;17&nbsp;августа по&nbsp;1&nbsp;сентября в&nbsp;New Millennium Concept Store будет проходить распродажа одежды, обуви и&nbsp;аксессуаров со&nbsp;скидками до&nbsp;50%.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock ">
                        <a href="/ru/2018/04/08/water/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2015/04/IxU6mL14302946454230_m.jpg" width="180" height="120" alt="В Ташкенте планируются отключения воды">
                        </a>
                        <div class="nt">
                            <div class="ndt">8 апреля 2018, 10:19&nbsp;<span class="ico-comm"><i class="fas fa-comment" aria-hidden="true"></i><span>19</span></span></div>
                            <h3>
                                <a href="/ru/2018/04/08/water/">В&nbsp;Ташкенте планируются отключения воды</a>
                            </h3>
                            <p>В&nbsp;период с&nbsp;9&nbsp;апреля по&nbsp;16&nbsp;мая в&nbsp;Ташкенте планируются отключения холодной воды.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                                    <div class="nblock last">
                        <a href="/ru/2013/03/27/water/" class="nimg">
                            <img class="lazy" data-src="https://www.gazeta.uz/media/img/2008/10/257_m.jpg" width="180" height="120" alt="В ряде районов Ташкента будет отсутствовать холодная вода">
                        </a>
                        <div class="nt">
                            <div class="ndt">27 марта 2013, 23:44&nbsp;<span class="ico-comm"><i class="fas fa-comment" aria-hidden="true"></i><span>3</span></span></div>
                            <h3>
                                <a href="/ru/2013/03/27/water/">В&nbsp;ряде районов Ташкента будет отсутствовать холодная вода</a>
                            </h3>
                            <p>Предприятие «Сувсоз» сообщило о&nbsp;предстоящих в&nbsp;апреле-мае отключениях холодной воды и&nbsp;понижении давления в&nbsp;связи с&nbsp;ремонтно-профилактическими работами на&nbsp;водоводах.</p>
                        </div>
                        <div class="clear"></div>
                    </div>
                            </div>
        		<div class="pagination">
			
    <div class="arrows">
                    <span class="prev">Предыдущая</span>
                            <a target="_self" class="next" href="/ru/search?q=%D0%BF%D1%80&amp;sort=relevance&amp;page=2" rel="next">Следующая</a>
            </div>

    <div class="digits">
    <span class="ut">
                                    <b>1</b>
                                                <a target="_self" href="/ru/search?q=%D0%BF%D1%80&amp;sort=relevance&amp;page=2">2</a>
                                                <a target="_self" href="/ru/search?q=%D0%BF%D1%80&amp;sort=relevance&amp;page=3">3</a>
                                                <a target="_self" href="/ru/search?q=%D0%BF%D1%80&amp;sort=relevance&amp;page=4">4</a>
                                                <a target="_self" href="/ru/search?q=%D0%BF%D1%80&amp;sort=relevance&amp;page=6">…</a>
                                                <a target="_self" href="/ru/search?q=%D0%BF%D1%80&amp;sort=relevance&amp;page=8">8</a>
                        </span>
    </div>
		</div>
        <div class="clear"></div>
	</div>
</div>

<div class="rightContainer">
				<div class="popOnWeekBlock">
						<h4>Популярное</h4>
			<ul>
								<li>
					<a href="/ru/2023/11/09/currency/">«Отката назад не&nbsp;будет». Центробанк Узбекистана&nbsp;— о&nbsp;поправках в&nbsp;правила конвертации валюты</a>&nbsp;<!--comm--><small>(<a href="/ru/2023/11/09/currency/#comments">2</a>)</small>				</li>
								<li>
					<a href="/ru/2023/11/10/fix-price/">Fix Price отмечает день рождения и&nbsp;запускает скидки</a>&nbsp;<!--comm--><small>(<a href="/ru/2023/11/10/fix-price/#comments">1</a>)</small>				</li>
								<li>
					<a href="/ru/2023/11/09/brutto/">Ещё 10 автобусных маршрутов Ташкента перешли на&nbsp;брутто-контракты</a>&nbsp;<!--comm--><small>(<a href="/ru/2023/11/09/brutto/#comments">2</a>)</small>				</li>
								<li>
					<a href="/ru/2023/11/10/incident-mosque/">Мужчина скончался во&nbsp;время пятничной молитвы в&nbsp;Ташкенте</a>&nbsp;<!--comm--><small>(<a href="/ru/2023/11/10/incident-mosque/#comments">2</a>)</small>				</li>
								<li>
					<a href="/ru/2023/11/08/currency-reaction/">Снова те&nbsp;же&nbsp;грабли. Колонка Карена Срапионова о&nbsp;планах ограничить конвертацию валюты юрлицами</a>&nbsp;<!--comm--><small>(<a href="/ru/2023/11/08/currency-reaction/#comments">7</a>)</small>				</li>
							</ul>
					</div>
	
<div class="mar-10"></div>
<div class="floating_banner_placeholder" style="display: none;"></div>
<div class="banner300x500 floating_banner">

    
                                                <div class="bVerticalRectangle">
                    <iframe loading="lazy" src="https://oa.afishamedia.net/www/delivery/afr.php?zoneid=11&amp;source=search" frameborder="0" scrolling="no" width="300" height="500">
                        <a href='https://oa.afishamedia.net/www/delivery/ck.php?n=ade8573d' target='_blank'>
                            <img src='https://oa.afishamedia.net/www/delivery/avw.php?zoneid=11&amp;n=ade8573d&amp;source=search' loading='lazy' border='0' alt='' />
                        </a>
                    </iframe>
                </div>
                            <ul class="banner_links">
        <li><a href="https://docs.google.com/presentation/d/1oi6Qyz-SLS4JfxElibQgopCfFWMa5-KQ-_GhqmG5Rpw/" target="_blank">Медиакит</a></li>
        <li><a href="https://www.gazeta.uz/ru/reklama">Цены</a></li>
        <li><a href="https://www.gazeta.uz/ru/contact">Контакты</a></li>
    </ul>
</div>

<div id="floating_banner_preserve">
    
    </div>
<div style="clear: both;"></div>
</div>

                <div class="sectionLine"><br></div>
        <div id="footer">
            <div class="copyright">
                <p>© 2008-2023 «Газета.uz»</p>
				<p>Новости для людей.</p>
                <p class="spelling-report">Нашли ошибку?<br>Нажмите Ctrl+Enter</p>
            </div>

            <div class="menuandterms">
                <div class="footer-menu">
                    <a href="/ru/reklama/">Реклама</a>
                    <a href="/ru/contact/">Контакты</a>
                    <a href="/ru/about/">О сайте</a>
                                        <a href="https://docs.google.com/forms/d/e/1FAIpQLSdhU1dTDFfckVBBCyZP7XN1Dgdy9N1Qy5mo6Su0KfXMuRzJlA/viewform" target="_blank">Вакансии</a>
                                        <a href="/ru/privacy/">Политика конфиденциальности</a>
                    <br>
                    <a href="/ru/terms/">Использование материалов</a>
                    <a href="/ru/feeds/">RSS</a>
                    <a href="https://telegram.me/gazetauz" target="_blank">Telegram</a>
                    <a href="https://instagram.com/gazetauzb/" target="_blank">Instagram</a>
                    <a href="https://www.youtube.com/@GazetaUzb" target="_blank">YouTube</a>
                    <a href="https://facebook.com/gazetauz/" target="_blank">Facebook</a>
                    <a href="https://twitter.com/gazeta_uz/" target="_blank">Twitter</a>
                    <a href="javascript:void(0)" class="js-push-button" data-lang="ru" style="display: none;">Включить уведомления</a>
                </div>
                <br>
                <div class="clear"></div>
                <p class="terms">
                    Воспроизводство, копирование, тиражирование, распространение и иное использование информации с сайта «Газета.uz» возможно только с предварительного письменного разрешения редакции.                </p>
                <p class="partnerof">
                    Регистрация электронного СМИ: №0460 от 13 августа 2019 года.<br>
		Учредитель: ООО «Afisha Media»<br>
		Главный редактор: Атаджанов Азамат Юсупбаевич<br>
		Адрес: 100007, Ташкент, ул. Паркент, 26А<br>
		Почта: <a href="mailto:info@gazeta.uz">info@gazeta.uz</a>
		                </p>
            </div>
            <div class="footer-right">
                <p class="censor">
                    <span>18+</span>
                </p>
                <p class="fButtons"></p>
            </div>
            <div class="clear"></div>
			        </div>

                <div class="modal modal-md" id="spelling-notification-modal" aria-hidden="false">
            <div class="modal-overlay" tabindex="-1" data-micromodal-close="">
                <div class="modal-container">
                    <div class="modal-header">
                        <button class="btn-flat btn-transparent modal-close" data-micromodal-close=""><i class="fas fa-times" data-micromodal-close=""></i></button>
                        <h4>Нашли ошибку в тексте?</h4>
                    </div>
                    <div class="modal-content">
                        <div class="before-submit">
                            <form id="spelling-notification-form" action="/ru/spelling/submit" method="POST" data-recaptcha-site-key="6Ldtcp4UAAAAAMJsORC1qkWiW6l0rPDuI62bLCUD">
                                <blockquote id="spelling-notification-text" data-max-length="100"></blockquote>
                                <input id="spelling-notification-comment" type="hidden" data-max-length="200" value="">
                                                                <div class="errors" style="display: none;" data-long-comment-error="Текст комментария не должен превышать 200 символов." data-long-text-error="Пожалуйста выделите не более 100 символов и повторите попытку." data-generic-error="Произошла ошибка. Попробуйте отправить сообщение позже или свяжитесь с администрацией сайта."></div>
                                <button class="btn-brand-color" type="submit">Отправить</button>
                            </form>
                        </div>
                        <div class="after-submit">
                            <p>Спасибо. Мы получили ваше сообщение и исправим ошибку в ближайшее время.</p>
                            <br>
                            <button class="btn-brand-color" data-micromodal-close="">Продолжить</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        
                    <script src="https://www.gstatic.com/firebasejs/5.3.0/firebase-app.js"></script>
            <script src="https://www.gstatic.com/firebasejs/5.3.0/firebase-messaging.js"></script>
            <script src="/js/chromepush/main.js?r=v230925"></script>
        
        <script src="/js/main.js?r=v230925"></script>
        <script>
            var recaptchaOnloadCallback = function() {
                document.getElementsByTagName('body')[0].setAttribute('data-recaptcha-loaded', '1');
                document.dispatchEvent(new Event('recaptcha-loaded'));
            };
        </script>
        <script src="https://www.google.com/recaptcha/api.js?onload=recaptchaOnloadCallback&amp;render=6Ldtcp4UAAAAAMJsORC1qkWiW6l0rPDuI62bLCUD" async="" defer=""></script>
        <script src="/js/micromodal.min.js?r=v230925"></script>
        <script src="/js/spelling.js?r=v230925"></script>
        
            </div>
    
<div id="login_modal_mask"> </div>
<div id="login_modal">
    <button class="close-modal head-close-modal"><i class="fas fa-times" aria-hidden="true"></i></button>
    <div class="head-modal-text">Авторизация на Газета.uz</div>
    <p>Авторизуйтесь на сайте, чтобы получить доступ к дополнительным возможностям.</p>
    <p id="login_modal_msg" class="form-message"></p>

    <div class="modal-footer" id="login_modal_footer_oauth">
        <div id="login_modal_oauth">
            <div class="oauth__wrapper telegram__ico" data-mode="auth" data-app="5502697877" data-telegram-login="GazetauzAssistantBot" data-size="large" data-userpic="false" data-type="telegram">

                <div class="oauth__wrapper-ico">
                    <i class="fab fa-telegram oauth__wrapper_ico" style="display: inline;"></i>
                    <i class="fas fa-spinner oauth__wrapper_spinner" style="display: none;"></i>
                </div>
                <div class="oauth__wrapper-label">Telegram</div>
            </div>

            <div class="oauth__wrapper facebook__ico" data-mode="auth" data-app="501898193157496" data-type="facebook">

                <div class="oauth__wrapper-ico">
                    <i class="fab fa-facebook oauth__wrapper_ico" style="display: inline;"></i>
                    <i class="fas fa-spinner oauth__wrapper_spinner" style="display: none;"></i>
                </div>
                <div class="oauth__wrapper-label">Facebook</div>
            </div>

            <div class="oauth__wrapper google__ico" data-mode="auth" data-app="31754806006-fvjglchndkc8ngehpigvhe9ij63f5afs.apps.googleusercontent.com" data-type="google">

                <div class="oauth__wrapper-ico">
                    <i class="fab fa-google oauth__wrapper_ico" style="display: inline;"></i>
                    <i class="fas fa-spinner oauth__wrapper_spinner" style="display: none;"></i>
                </div>
                <div class="oauth__wrapper-label">Google</div>
            </div>
        </div>
        <div class="clear"></div>
    </div>
</div>
    <script src="/js/auth.js?r=v230925"></script>
    <script src="/js/scripts.js?r=v230925" gapi_processed="true"></script>
</div>


<iframe id="ssIFrame_google" sandbox="allow-scripts allow-same-origin allow-storage-access-by-user-activation" style="position: absolute; width: 1px; height: 1px; inset: -9999px; display: none;" aria-hidden="true" frame-border="0" src="https://accounts.google.com/o/oauth2/iframe#origin=https%3A%2F%2Fwww.gazeta.uz&amp;rpcToken=315468933.3977454"></iframe><div><div class="grecaptcha-badge" data-style="bottomright" style="width: 256px; height: 60px; position: fixed; visibility: hidden; display: block; transition: right 0.3s ease 0s; bottom: 14px; right: -186px; box-shadow: gray 0px 0px 5px; border-radius: 2px; overflow: hidden;"><div class="grecaptcha-logo"><iframe title="reCAPTCHA" width="256" height="60" role="presentation" name="a-7insalt28u86" frameborder="0" scrolling="no" sandbox="allow-forms allow-popups allow-same-origin allow-scripts allow-top-navigation allow-modals allow-popups-to-escape-sandbox allow-storage-access-by-user-activation" src="https://www.google.com/recaptcha/api2/anchor?ar=1&amp;k=6Ldtcp4UAAAAAMJsORC1qkWiW6l0rPDuI62bLCUD&amp;co=aHR0cHM6Ly93d3cuZ2F6ZXRhLnV6OjQ0Mw..&amp;hl=ru&amp;v=fGZmEzpfeSeqDJiApS_XZ4Y2&amp;size=invisible&amp;cb=3t14gai7z4ib"></iframe></div><div class="grecaptcha-error"></div><textarea id="g-recaptcha-response-100000" name="g-recaptcha-response" class="g-recaptcha-response" style="width: 250px; height: 40px; border: 1px solid rgb(193, 193, 193); margin: 10px 25px; padding: 0px; resize: none; display: none;"></textarea></div><iframe style="display: none;"></iframe></div><script type="text/javascript" id="">!function(b,e,f,g,a,c,d){b.fbq||(a=b.fbq=function(){a.callMethod?a.callMethod.apply(a,arguments):a.queue.push(arguments)},b._fbq||(b._fbq=a),a.push=a,a.loaded=!0,a.version="2.0",a.queue=[],c=e.createElement(f),c.async=!0,c.src=g,d=e.getElementsByTagName(f)[0],d.parentNode.insertBefore(c,d))}(window,document,"script","https://connect.facebook.net/en_US/fbevents.js");fbq("init","440076653520103");fbq("set","agent","tmgoogletagmanager","440076653520103");fbq("track","PageView");</script>
<noscript><img height="1" width="1" style="display:none" src="https://www.facebook.com/tr?id=440076653520103&amp;ev=PageView&amp;noscript=1"></noscript><div id="fb-root" class=" fb_reset"><div style="position: absolute; top: -10000px; width: 0px; height: 0px;"><div></div></div></div><script type="text/javascript" id="">(function(){try{window.setTimeout(function(){dataLayer.push({event:"afterLoad"})},500)}catch(a){}})();</script>
<script type="text/javascript" id="">(function(a,e,f,g,b,c,d){a[b]=a[b]||function(){(a[b].a=a[b].a||[]).push(arguments)};a[b].l=1*new Date;c=e.createElement(f);d=e.getElementsByTagName(f)[0];c.async=1;c.src=g;d.parentNode.insertBefore(c,d)})(window,document,"script","https://mc.yandex.ru/metrika/tag.js","ym");ym(757564,"init",{clickmap:!0,trackLinks:!0,accurateTrackBounce:!0});</script>
<noscript><div><img src="https://mc.yandex.ru/watch/757564" style="position:absolute; left:-9999px;" alt=""></div></noscript>
</body></html>"##;

    let target_news = News{ 
        body: "Совсем скоро в&nbsp;Ташкенте состоится международная выставка-ярмарка CAEx Mebel &amp; Décor 2023. Для удобства посетителей организованы 50 бесплатных автобусов, на&nbsp;которых можно будет добраться до&nbsp;выставки.".to_string(), 
        reference_to_photo: "https://www.gazeta.uz/media/img/2023/09/wf2RKB16951062217783_m.jpg".to_string(), 
        time_string: "18 сентября 2023, 21:00".to_string(), 
        title: "В&nbsp;Ташкенте для посетителей CAEx Mebel &amp; Décor 2023 будут организованы 50 бесплатных автобусов".to_string() };
    
    let news = 
parce_news(html.to_string()) ;
       assert!(news.contains(&target_news))
}
