use timpl::*;

fn main() {
    println!("{}", template());
}

pub(crate) fn template() -> String {
    let profile = profile();

    let title = timpl! { Profile of { profile.name } };
    let description = timpl! { Profile of { profile.name } writen in Rust by { profile.name } };

    timpl! {

        <!doctype html>

        <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">

            <title>{ title }</title>
            <meta name="description" content={'"'}{ description }{'"'}>
            <meta name="author" content={'"'}{ profile.name }{'"'}>
        </head>

        <body>
            <h1>{ profile.name }</h1>
            <p>Wubba lubba dub-dub!</p>
            <h2>Occupations</h3>

            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Status</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        timpl_map_ln!(profile.occupations.iter(), item, {
                            <tr id={'"'}item-{ item.id }{'"'}>
                                <td>{ item.name }</td>
                                <td>{ item.status.unwrap_or("-") }</td>
                            </tr>
                        })
                    }
                </tbody>
            </table>
        </body>
        </html>

    }
}

fn profile() -> Profile<'static> {
    Profile {
        name: "Rick Sanchez",
        occupations: vec![
            Occupation {
                id: 1,
                name: "Scientist",
                status: Some("PhD in interdimensional physics"),
            },
            Occupation {
                id: 2,
                name: "Inventor",
                status: None,
            },
            Occupation {
                id: 3,
                name: "Resistance fighter",
                status: Some("Leader of the Rick's Rebellion"),
            },
            Occupation {
                id: 4,
                name: "Arms dealer",
                status: None,
            },
            Occupation {
                id: 5,
                name: "Store owner",
                status: Some("briefly"),
            },
            Occupation {
                id: 6,
                name: "Leader of The Council of Ricks",
                status: Some("formerly"),
            },
        ],
    }
}

struct Profile<'a> {
    name: &'a str,
    occupations: Vec<Occupation<'a>>,
}

struct Occupation<'a> {
    id: u32,
    name: &'a str,
    status: Option<&'a str>,
}
