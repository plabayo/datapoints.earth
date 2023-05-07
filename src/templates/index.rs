use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh") {
            div(style = "max-width: 780px;") {
                h1 { "🌎    datapoints.earth 🌍" }
                p {
                    "Built with "
                    a (href = "https://github.com/plabayo/netscrape") { "FOSS" }
                    " by "
                    a (href = "https://plabayo.tech" ) { "Plabayo" }
                    " with as goal to liberate the data about our earth to help others with helping our earth."
                }
                h2 { "What❓" }
                p {
                    "We specialize in extracting data from web platforms and making it available to everyone. "
                    "Data from APIs are possible as well upon request and with permission, but data otherwise difficult to reach is where we shine."
                    "All data is extracted periodically and the snapshots remain available for everyone \"forever\"."
                }
                h2 { "💬    Call for help!" }
                p {
                    "Please reach out to us by email at "
                    a (href = "mailto:contact@datapoints.earth") { "contact@datapoints.earth" }
                    " if you are an organization, student or academic that needs data from a web platform. "
                    "We are happy to help you out as long as you clearly motivate why you need the data and what you will do with it. "
                    "Due to our limited resources we have to prioritize based on impact and feasibility."
                }
                p {
                    "You can also contact us in case you want to help out with the project "
                    "or have an API available with data that you wish to integrate into our platform."
                }
                p {
                    "Do you already use our data? Please share us your story and we can see to add it to our testimonials."
                }
                h2 { "👷‍♀️    Work In Progress" }
                p {
                    "This platform is still in its early stages and not yet available to the public. "
                    "We are working hard to make it available as soon as possible."
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "📡 data 🌏" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
