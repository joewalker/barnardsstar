#![allow(dead_code)]

mod tests;
mod types;

fn main() {
    // let mut parser = Parser::new(ACTIVE_SESSIONS);
    //
    // loop {
    //     let parsed = parser.read();
    //
    //     if parsed.is_none() {
    //         break;
    //     }
    //
    //     println!("{:?}", parsed.unwrap());
    // }
}

const ACTIVE_SESSIONS: &'static str = "#{
    :find ?id ?reason ?ts
    :in $
    :where
    [?id :session/startReason ?reason ?tx]
    [?tx :db/txInstant ?ts]
    (not-join [?id]
        [?id :session/endReason _])
}";

const ENDED_SESSIONS: &'static str = "
    :find ?id ?endReason ?ts
    :in $
    :where
    [?id :session/endReason ?endReason ?tx]
    [?tx :db/txInstant ?ts]
";

const STARRED_PAGES: &'static str = "
    :find '[?url ?title ?starredOn]
    :in (if since '[$ ?since] '[$])
    :where where
";

const SAVED_PAGES: &'static str = "
    :find ?page ?url ?title ?excerpt
    :in $
    :where
    [?save :save/page ?page]
    [?save :save/savedAt ?instant]
    [?page :page/url ?url]
    [(get-else $ ?save :save/title \"\") ?title]
    [(get-else $ ?save :save/excerpt \"\") ?excerpt]
";

const PAGES_MATCHING_STRING_1: &'static str = "
    :find '[?url ?title]
    :in '[$]
    :where [
        [(list 'fulltext '$ #{:page/url :page/title} string) '[[?page]]]
        '[(get-else $ ?page :page/url \"\") ?url]
        '[(get-else $ ?page :page/title \"\") ?title]
    ]
";

const PAGES_MATCHING_STRING_2: &'static str = "
    :find '[?url ?title ?excerpt]
    :in '[$]
    :where [
        [(list 'fulltext '$ #{:save/title :save/excerpt :save/content} string) '[[?save]]]
        '[?save :save/page ?page]
        '[?page :page/url ?url]
        '[(get-else $ ?save :save/title \"\") ?title]
        '[(get-else $ ?save :save/excerpt \"\") ?excerpt]
    ]
";

const VISITED: &'static str = "
    :find '[?url ?title (max ?time)]
    :in (if since '[$ ?since] '[$])
    :where where
";

const FIND_TITLE: &'static str = "
    :find ?title .
    :in $ ?url
    :where
    [?page :page/url ?url]
    [(get-else $ ?page :page/title \"\") ?title]
";
