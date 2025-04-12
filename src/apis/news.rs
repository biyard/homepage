use bdk::prelude::*;
use server_fn::codec::GetUrl;

use crate::News;

#[server(endpoint = "/news", input = GetUrl)]
pub async fn get_news() -> Result<Vec<News>, ServerFnError> {
    Ok (
        vec![
            News {
                category: "General".to_string(),
                title: "Lörem ipsum tide metakrati. ".to_string(),
                contents: "Lörem ipsum biode vöräsöktig. Tekyre nysm. Dil trisam. Okårat trevöngen liheten. Ontopod nyll. Domoktig apograf otekrod och kåledes och makroteng. Pladugt spertad vaninas omöbelt ac-förkylning. Antemuligen sösam kagen ifall båren. Egolig nixa, egopylig. Otån tifäs preligen. Suprav. Douche benåning iligt osm.".to_string(),
                image: "https://letsenhance.io/static/73136da51c245e80edc6ccfe44888a99/1015f/MainBefore.jpg".to_string(),
                ..Default::default()
            };4
        ]
    )
}
