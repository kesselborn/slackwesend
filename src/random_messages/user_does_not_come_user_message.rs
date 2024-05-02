use rand::seq::SliceRandom;
use rocket::error;
use rocket::serde::Serialize;
use tinytemplate::TinyTemplate;

pub fn user_does_not_come_user_message(weekday: &str) -> String {
    let sentences: Vec<&str> = vec![
    "Ach wirklich? Schon wieder abgesagt? Na, das überrascht jetzt aber niemanden hier.",
    "Oh, du kommst am {weekday} doch nicht? Was für eine unerwartete Wendung... sagte niemand, jemals.",
    "Na toll, du sagst ab. Hier bricht jetzt natürlich das große Chaos aus... oder auch nicht.",
    "Natürlich, sag nur ab. Es ist ja nicht so, als hätten wir uns gefreut oder so.",
    "Ach, das ganze Lob von vorhin? War sowieso nur Show. Gut, dass du nicht kommst.",
    "Also doch keine lebendige Atmosphäre am {weekday}? Keine Überraschung, wir sind es gewohnt.",
    "Du sagst ab? Schockierend. Wirklich. Mein ganzes Leben ist jetzt anders... oder auch nicht.",
    "Oh, eine Absage. Wie originell von dir. Hoffe, dein {weekday} ist spannender als deine Ausreden.",
    "Deine Absage ist eingetroffen, und die Enttäuschung... hält sich in Grenzen.",
    "Ganz großes Kino, deine Absage. Wir machen hier ohne dich eine noch bessere Party!",
    "Schon klar, du hast sicher was Besseres vor. Wie jedes Mal. Genieß deinen {weekday} allein!",
    "Du kommst nicht? Was für eine Überraschung. Wir hatten uns so darauf eingestellt... Nicht.",
    "Ohne dich wird der {weekday} im Büro erträglicher. Danke für deine Absage!",
    "Deine Absage hat uns so getroffen... Ach, wer macht hier eigentlich Witze? Niemand vermisst das Drama.",
    "Absage akzeptiert. Der {weekday} wird nun ein ruhiger Tag. Danke dafür.",

    "Schade, dass du am {weekday} doch nicht kommst. Wir hatten uns schon gefreut!",
    "Oh, du sagst ab? Nun, wir werden dich am {weekday} vermissen!",
    "Das ist wirklich enttäuschend, aber wir hoffen, dass du einen guten Grund hast. Bis zum nächsten Mal!",
    "Deine Absage ist angekommen. Schade, aber wir machen das Beste daraus!",
    "Ach, du kommst am {weekday} nicht. Naja, vielleicht beim nächsten Mal dann.",
    "Du sagst ab? Okay, hoffentlich können wir uns bald wiedersehen!",
    "Also doch keine {username}-Show am {weekday}? Schade, wir hatten uns schon darauf eingestellt.",
    "Na gut, dann eben nicht {username} am {weekday}. Wir werden irgendwie klarkommen.",
    "Ohne {username} am {weekday} – das Büro wird ein bisschen ruhiger sein. Vielleicht auch nicht schlecht.",
    "Du sagst ab? Wir hatten gehofft, dich zu sehen, aber Verpflichtungen gehen vor.",
    "Ohne dich wird der {weekday} ein bisschen grauer, aber wir verstehen es.",
    "Ach, wirklich? Ohne {username} am {weekday}? Wir werden deine Energie vermissen!",
    "Schade, dass du deine Pläne ändern musstest. Hoffentlich klappt es das nächste Mal!",
    "Wir hatten uns auf deine Geschichten gefreut, aber deine Absage verstehen wir natürlich auch.",
    "Kein {username} am {weekday}? Das ist bedauerlich, aber Gesundheit und Familie gehen vor.",
    "Deine Absage hat uns erreicht. Schade, aber wir wissen, dass manchmal andere Dinge Priorität haben.",
    "Also sehen wir dich am {weekday} nicht. Hoffentlich ist alles in Ordnung bei dir!",
    "Du kommst nicht? Schade, aber wir erwarten dich beim nächsten Mal mit offenen Armen!",
    "Wir hatten uns schon auf einen tollen {weekday} mit dir eingestellt. Hoffentlich bist du bald wieder dabei!",
    "Kein {username} am {weekday}? Nun, wir sparen uns die guten Witze für dein nächstes Mal auf.",

    "Du kommst am {weekday} nicht? Schade, die Büropflanzen werden dich vermissen!",
    "Ohne dich ist der {weekday} nur halb so lustig. Jetzt müssen wir wohl alleine lachen.",
    "Du sagst ab? Wir hatten doch extra den roten Teppich bestellt!",
    "Ein {weekday} ohne dich? Wie soll das Büro nur ohne seine tägliche Dosis {username} überleben?",
    "Nicht da am {weekday}? Und wer bringt uns jetzt zum Lachen? Der Drucker?",
    "Ohne {username} am {weekday}? Okay, Party ist abgesagt, Leute!",
    "Schade, dass du nicht kommst. Ich hatte schon extra meinen neuen Hut aufgesetzt!",
    "Du fehlst am {weekday}? Nun, wir werden irgendwie versuchen, ohne deine Witze klarzukommen.",
    "Absage für den {weekday} erhalten. Wir werden den Tag in Trauerkleidung verbringen.",
    "Kein {username}, kein Kuchen. Die Regeln sind einfach am {weekday}.",
    "Am {weekday} nicht da? Na gut, dann heben wir die spannenden Projekte für später auf.",
    "Du sagst den {weekday} ab? Jetzt muss ich wohl alleine den Kaffee trinken!",
    "Ohne dich wird der {weekday} so farblos sein. Also quasi Grau in Grau.",
    "Ohne {username} am {weekday}? Die Kaffeemaschine wird so enttäuscht sein.",
    "Ein {weekday} ohne dich ist wie eine Tastatur ohne Enter-Taste – irgendwie unvollständig.",
    "Du bist am {weekday} nicht da? Schon klar, du brauchst eine Pause von uns Chaoten!",
    "Also ohne {username} am {weekday}. Werde wohl meine eigenen Scherze erzählen müssen – oh weh!",
    "Am {weekday} nicht im Büro? Na hoffentlich hast du eine gute Ausrede, wie z.B. ein Lottogewinn!",
    "Du sagst ab? Und ich hatte mich schon so auf unser Mittagessen gefreut. Jetzt muss ich wohl allein essen!",
    "Du kommst nicht? Gut, dann park ich morgen auf deinem Platz!",
    "Ach nein, du fehlst am {weekday}? Werde meinen Tratschklatsch wohl selbst erfinden müssen.",
    "Kein {username} bedeutet mehr Kuchen für den Rest von uns. Jeder Nachteil hat seine Vorteile!",
    "Oh, du sagst ab? Dann streiche ich dich von der Liste für den besten Mitarbeiter des Monats!",
    "Am {weekday} nicht da? Schade, die Gerüchteküche wird ohne dich nicht brodeln.",
    "Du fehlst am {weekday}? Ach, ich werde einfach ein Bild von dir aufstellen und so tun als ob.",
    "Nicht da? Dann werde ich wohl deine Pflanze adoptieren müssen. Hoffentlich spricht sie auch mit mir.",
    "Ohne {username} am {weekday}? Jetzt muss ich wohl zweimal so viel arbeiten – oder auch nicht.",
    "Du kommst am {weekday} nicht? Die gute Nachricht: Mehr Kaffee für mich!",
    "Kein {username} am {weekday} bedeutet eine Pause von deinen furchtbaren Wortwitzen. Endlich Erholung!"

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
