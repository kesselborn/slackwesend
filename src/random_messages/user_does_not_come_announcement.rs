use rand::seq::SliceRandom;
use rocket::error;
use rocket::serde::Serialize;
use tinytemplate::TinyTemplate;

pub fn user_does_not_come_announcement(username: &str, weekday: &str) -> String {
    let sentences: Vec<&str> = vec![
    "Schade, dass {username} am {weekday} nicht ins Büro kommt. 🙁",
    "Ich bin so enttäuscht, dass {username} {weekday} nicht da sein wird. 😔",
    "Schade, dass der {weekday} ohne {username} im Büro sein wird. 😞",
    "Morgen ist {weekday} und leider wird {username} nicht hier sein. 😕",
    "Ich bin sauer, dass {username} ausgerechnet am {weekday} fehlt. 😠",
    "Der {weekday} wird ohne {username} im Büro nicht dasselbe sein. 😟",
    "Ich hatte gehofft, dass {username} {weekday} da sein würde, aber leider nicht. 😩",
    "{weekday} ohne {username} fühlt sich irgendwie falsch an. 😢",
    "Schade, dass der {weekday} ohne {username}s Anwesenheit stattfinden muss. 🥺",
    "Ich kann es nicht fassen, dass ausgerechnet {weekday} der Tag ist, an dem {username} nicht da sein wird. 😫",
    "Schade, dass ich am {weekday} auf {username} verzichten muss. 😞",
    "Der {weekday} wird ohne {username} so langweilig sein. 🙁",
    "Ich hatte mich schon so auf den {weekday} mit {username} gefreut... 😔",
    "Der {weekday} ohne {username} ist wie ein Tag ohne Sonnenschein. ☁️",
    "Es ist einfach nicht dasselbe ohne {username} am {weekday}. 😕",
    "Der {weekday} wird ohne {username} einfach nicht perfekt sein. 🙅‍♂️",
    "Schweren Herzens muss ich akzeptieren, dass {username} am {weekday} nicht da ist. 💔",
    "Ohne {username} am {weekday} fühle ich mich wie ein verlorener Schaf. 🐑",
    "Der {weekday} ohne {username} ist wie ein Kaffee ohne Milch. ☕️",
    "Warum muss {username} gerade am {weekday} fehlen? 😭",
    "Jetzt kommt {username} schon wieder nicht ins Büro, immer diese Ausreden an einem {weekday}!",
    "Schade, dass {username} {weekday} nicht zur Arbeit kommt, hätte ich gerne mal wieder getroffen.",
    "{weekday}s ist doch ein normaler Arbeitstag, wieso muss {username} unbedingt frei haben?",
    "Ich hatte gehofft, {username} {weekday} endlich mal wieder zu sehen, aber nein ... bleibt zu Hause.",
    "Als ob {username} nicht schon genug freie Tage hätte, jetzt auch noch {weekday} frei sein.",
    "Ich finde es unhöflich, dass {username} uns ohne Vorwarnung am {weekday} im Stich lässt.",
    "Ich kann es nicht verstehen, warum {username} ausgerechnet am {weekday} Urlaub nehmen muss, das ist doch ein ganz normaler Arbeitstag.",
    "Wenn {username} schon nicht ins Büro kommen will, dann wäre eine vernünftige Entschuldigung angebracht.",
    "Es ist wirklich ärgerlich, dass {username} ausgerechnet am {weekday} nicht im Büro sein kann.",
    "Man kann sich auf {username} einfach nicht verlassen, {weekday} wieder nicht im Büro.",
    "Ich kann es nicht glauben, dass {username} am {weekday} einfach so frei nimmt und uns im Stich lässt.",
    "{username} fehlt schon wieder am {weekday}, das ist wirklich unverantwortlich.",
    "Ich hatte mich schon so auf {username} am {weekday} gefreut, jetzt kann ich meine Pläne wieder umwerfen.",
    "Es ist wirklich schade, dass {username} am {weekday} nicht im Büro ist, wir werden ihn vermissen.",
    "Ich finde es respektlos, dass {username} am {weekday} einfach nicht zur Arbeit erscheint.",
    "Ich hätte gedacht, {username} würde es besser wissen, dass man am {weekday} nicht einfach frei nimmt.",
    "Ich verstehe nicht, warum {username} am {weekday} nicht ins Büro kommen kann, es gibt doch keinen triftigen Grund dafür.",
    "Ich bin wirklich enttäuscht, dass {username} am {weekday} nicht ins Büro kommt.",
    "Es ist unglaublich, dass {username} ausgerechnet am {weekday} keine Zeit für die Arbeit hat, das ist wirklich frustrierend.",
    "Oh nein! Am {weekday} wird {username} nicht da sein. Das Büro wird leer und traurig sein. 😢",
    "Dunkelheit im Büro am {weekday}, denn {username} bleibt uns fern. 💔",
    "Verloren ohne {username} am {weekday} – das Büro fühlt sich leer an. Wo bist du? 😞",
    "Ein {weekday} ohne {username} ist wie ein Himmel ohne Sterne – dunkel und trist. 🌑",
    "Das Büro weint am {weekday}, denn ohne {username} ist alles grau. 😭",
    "Alarm! {username} kommt am {weekday} nicht! Wer soll jetzt das Büro retten? 😡",
    "Unvorstellbar! Ein {weekday} ohne {username} – das ist ein Fehler im System! 🚨",
    "Katastrophe! Ohne {username} hat der {weekday} keinen Sinn. Wo bleibst du? 😖",
    "Tragödie schlägt zu: {username} fehlt uns am {weekday}. Warum nur, warum? 😫",
    "Am {weekday} bleibt {username} weg. Das Büro verliert seinen besten Held. 😥",
    "Verzweiflung pur: Kein {username} am {weekday}. Wie sollen wir weitermachen? 😟",
    "Ein Schrei der Verzweiflung: Wo ist {username} am {weekday}? Wir brauchen dich! 😧",
    "Der {weekday} wird bitter, denn {username} lässt uns allein. 😒",
    "Ein {weekday} ohne {username}? Das ist wie ein Computer ohne Bildschirm. 🖥️💔",
    "Verwirrung und Wut – {username} kommt am {weekday} nicht, und nichts macht mehr Sinn. 😠",
    "Leere Hallen am {weekday}, denn {username} ist nicht da. Es ist so still. 🌌",
    "Wo bist du, {username}? Deine Abwesenheit am {weekday} lässt das Büro kalt zurück. ❄️",
    "Am {weekday} bleibt {username} fern – das Büro hat einen dunklen Schatten. 🌑",
    "Wütend und frustriert, weil {username} am {weekday} uns verlässt. Wie kannst du nur? 😡",
    "Verzweiflung am {weekday}: {username} ist nicht hier, und nichts ist klar. 🌫️",
    "Ein {weekday} ohne {username} – ein Tag ohne Freude. 😕",
    "Warnung: Geringe Produktivität erwartet, da {username} am {weekday} fehlt. 📉",
    "Oh, die Stille! Ohne {username} am {weekday} ist das Büro wie verlassen. 🏚️",
    "Es ist offiziell: {weekday} ist der schlechteste Tag, {username} kommt nicht. 😞",
    "Ein Rätsel: Wie überleben wir den {weekday} ohne {username}? Unmöglich! 🤯",
    "Traurige Nachrichten: Am {weekday} bleibt der Stuhl von {username} leer. 😢",
    "Es ist ein Notfall! {username} kommt am {weekday} nicht – Alarmstufe Rot! 🔴"
    ];

    let mut rng = rand::thread_rng();
    let random_sentence = sentences.choose(&mut rng).unwrap();
    let mut tt = TinyTemplate::new();

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    struct TemplateData {
        username: String,
        weekday: String,
    }
    let data = TemplateData {
        username: username.to_string(),
        weekday: weekday.to_string(),
    };

    tt.add_template("", random_sentence).unwrap();

    match tt.render("", &data) {
        Ok(s) => s,
        Err(e) => {
            error!("error rendering '{random_sentence}': {e}");
            "DANIEL HAT ES VERBOCKT ... SOFORT ANSCHREIEN BITTE".to_string()
        }
    }
}
