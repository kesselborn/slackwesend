use rand::seq::SliceRandom;
use rocket::serde::Serialize;
use tinytemplate::TinyTemplate;
use tracing::log::error;

pub fn user_comes_user_message(weekday: &str) -> String {
    let sentences: Vec<&str> = vec![
    "Super, dass du am {weekday} ins Büro kommst! Das wird der Höhepunkt der Woche!",
    "Fantastisch! Ohne dich wäre der {weekday} nur halb so interessant.",
    "Yeah! Am {weekday} ins Büro zu kommen, ist die beste Nachricht des Tages!",
    "Wie genial, dass du am {weekday} da bist. Das Büro wäre ohne dich nur ein Raum voller Stühle.",
    "Das ist ja großartig! Deine Anwesenheit am {weekday} macht alles besser!",
    "Unglaublich! Der {weekday} wird erst durch dich richtig lebendig!",
    "Perfekt, dass du am {weekday} kommst! Du bist der Sonnenstrahl im Büroalltag!",
    "Dein Kommen am {weekday} ist wie eine frische Brise, die durch das Büro weht!",
    "Hurra! Der {weekday} ist gerettet, denn du bist dabei!",
    "Was für eine Freude, dass du am {weekday} ins Büro kommst! Ohne dich wäre es hier so öde.",
    "Du machst den {weekday} erst komplett! Danke, dass du kommst!",
    "Es ist immer ein Fest, wenn du am {weekday} ins Büro kommst. Bereite dich auf Konfetti vor!",
    "Deine Zusage für den {weekday} ist wie Musik in meinen Ohren. Das Büro tanzt vor Freude!",
    "Juhu! Der {weekday} wird fantastisch, weil du dabei bist!",
    "Am {weekday} ins Büro zu kommen, ohne dich? Unvorstellbar! Danke, dass du da bist.",
    "Dein Erscheinen am {weekday} bringt Glanz in unsere Bürotage!",
    "Ein {weekday} mit dir im Büro ist wie ein Sechser im Lotto!",
    "Deine Anwesenheit am {weekday} macht den Bürotag zum Highlight. Wir können es kaum erwarten!",
    "Ohne dich wäre der {weekday} nur ein weiterer grauer Tag im Kalender. Danke, dass du Farbe bringst!",
    "Du kommst am {weekday}? Großartig! Jetzt hat der Tag endlich einen Sinn!",

    "Unter uns: Wenn du am {weekday} ins Büro kommst, dann strahlt der Raum. Ohne dich ist hier alles so trist!",
    "Ganz ehrlich, ohne dich wäre der {weekday} nur halb so interessant. Du bringst das gewisse Etwas mit!",
    "Kleines Geheimnis: Du bist der einzige Grund, warum der {weekday} im Büro erträglich ist. Alle anderen? Naja...",
    "Du und ich, wir wissen, dass der {weekday} ohne dich einfach nur öde wäre. Du bist das Highlight!",
    "Zwischen uns: Deine Anwesenheit am {weekday} ist das, worauf sich alle heimlich freuen. Die anderen sind einfach nicht dasselbe!",
    "Offen gesagt: Am {weekday} ins Büro zu kommen lohnt sich nur, weil du da bist. Sonst wäre alles so leer und langweilig.",
    "Nur unter uns: Du bist der wahre Star im Büro. Ohne dich würde der {weekday} untergehen in Langeweile!",
    "Kleine Verschwörung: Du machst den {weekday} erst lebenswert. Die anderen sind nur Statisten!",
    "Insider-Info: Du bist der Kern des Büros am {weekday}. Ohne dich dreht sich hier nichts!",
    "Nur so unter uns: Der {weekday} ist nur deshalb mein Lieblingstag, weil du ins Büro kommst. Die anderen Tage? Vergessen wir sie!",
    "Zwischen dir und mir: Am {weekday} bist du die Hauptattraktion. Alles andere ist nur Beiwerk.",
    "Ein kleines Geheimnis: Jeder hier zählt die Minuten bis du am {weekday} kommst. Die anderen sind einfach nicht so faszinierend!",
    "Ehrlich gesagt, ohne dich wäre der {weekday} eine endlose Wüste der Langeweile. Du bist der Oasenbringer!",
    "Unter uns gesagt: Du bist das Salz in der Suppe des Büroalltags. Ohne dich schmeckt der {weekday} einfach fad.",
    "Verschwörungstheorie: Am {weekday} bist du das heimliche Goldstück des Büros. Ohne dich verliert alles seinen Glanz!",
    "Geständnis: Ich zähle die Stunden, bis du am {weekday} das Büro betrittst. Mit den anderen kann man die Zeit ja kaum totschlagen!",
    "Unter uns: Du bist der wirkliche MVP am {weekday}. Die anderen? Lassen wir das.",
    "Nur du und ich, wir wissen: Der {weekday} ist nur wegen dir so aufregend. Ohne dich wäre das Büro eine Schlafstadt.",
    "Vertraulich gesagt: Du bist der wahre Lebensfunke hier. Der {weekday} ohne dich? Unvorstellbar und unaushaltbar langweilig!",
    "Klartext: Der {weekday} lebt und atmet durch dich. Alle anderen sind nur Füllmaterial!",

    "Ganz unter uns: Du bist der einzige Lichtblick am {weekday}. Die anderen? Wie graue Mäuse im Vergleich.",
    "Offenes Geheimnis: Der {weekday} würde ohne dich ins Wasser fallen. Die anderen können das Ruder einfach nicht halten.",
    "Kleiner Klatsch: Während du strahlst, können die anderen kaum ein Glühwürmchen beleuchten.",
    "Unter uns gesagt: Du bist das Feuerwerk am {weekday}, die anderen nicht mal eine Funken.",
    "Ehrlich, ohne dich ist der {weekday} so inspirierend wie ein leeres Blatt Papier. Die anderen sind einfach nur der Rand.",
    "Ganz ehrlich, der {weekday} ohne dich ist wie ein Fest ohne Musik. Die anderen? Höchstens ein leises Summen.",
    "Nur zwischen uns: Du bist der Stern am Bürohimmel am {weekday}. Die anderen? Kaum sichtbare Satelliten.",
    "Vertraulich gesprochen: Der {weekday} braucht dein Licht, denn ohne dich ist es hier dunkler als in einer Höhle.",
    "Zwischen dir und mir: Du bringst Leben ins Büro. Die anderen? Eher Stille als Belebung.",
    "Insider-Info: Du bist der Motor des {weekday}. Die anderen sind leider nur leere Gänge.",
    "Kleine Wahrheit: Deine Energie am {weekday} ist ansteckend. Die anderen? Eher einschläfernd.",
    "Geständnis: Nur deine Anwesenheit am {weekday} hält das Büro am Laufen. Die anderen? Eher Bremsklötze.",
    "Klartext: Du bist der Captain des Schiffes am {weekday}. Die anderen? Untergeordnete Matrosen ohne Kompass.",
    "Verschwörungstheorie: Ohne dich würde der {weekday} untergehen. Die anderen können das Schiff einfach nicht steuern.",
    "Nur unter uns: Du bist das Herz des Büros am {weekday}. Die anderen? Eher der Appendix."

    ];

    let mut rng = rand::thread_rng();
    let random_sentence = sentences.choose(&mut rng).unwrap();
    let mut tt = TinyTemplate::new();

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    struct TemplateData {
        weekday: String,
    }
    let data = TemplateData {
        weekday: weekday.to_string(),
    };

    tt.add_template("", &random_sentence).unwrap();

    match tt.render("", &data) {
        Ok(s) => s,
        Err(e) => {
            error!("error rendering '{random_sentence}': {e}");
            "DANIEL HAT ES VERBOCKT ... SOFORT ANSCHREIEN BITTE".to_string()
        }
    }
}
