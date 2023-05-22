use rand::seq::SliceRandom;
use rocket::serde::Serialize;
use tinytemplate::TinyTemplate;
use tracing::log::error;

pub fn user_comes_announcement(username: &str, weekday: &str) -> String {
    let sentences: Vec<&str> = vec![
//announce auf 50 verschiedene arten, dass {username} am {weekday} ins büro kommt und feier das ab. du kannst auch emojis nutzen. ersetze {username} durch {username} und {weekday} durch {weekday}. Variiere auch die emojiis und die anzahl der emojis die du nutzt. die sprache ist deutsch
"Yay! 🥳 {username} ist am {weekday} im Büro! 🎉",
"Wir freuen uns, dass {username} an diesem {weekday} im Büro ist! 🙌🏼",
"Es ist großartig, dass {username} am {weekday} in das Büro zurückkehrt! 👏🏼",
"Endlich ist {username} wieder da! Wir freuen uns, ihn an diesem {weekday} im Büro zu sehen. 🤗",
"Es ist ein guter Tag, denn {username} wird am {weekday} im Büro sein! 🎊",
"🎉 {username} kommt am {weekday} ins Büro! 🥳",
"Es wird ein großartiger {weekday}, denn {username} wird im Büro sein! 😃",
"Wir haben Grund zum Feiern, denn {username} kehrt am {weekday} ins Büro zurück! 🎉👍🏼",
"Wir freuen uns, {username} am {weekday} im Büro zu begrüßen! 🙌🏼",
"Willkommen zurück, {username}! Wir sind begeistert, ihn am {weekday} im Büro zu sehen. 🤗",
"Wir können es kaum erwarten, {username} am {weekday} im Büro zu begrüßen! 🎊",
"Es ist großartig, dass {username} am {weekday} wieder im Büro ist! 😁",
"Wir sind aufgeregt, {username} an diesem {weekday} im Büro zu sehen! 🤩",
"🚨 Breaking News: {username} kommt am {weekday} ins Büro! 🚨",
"Hurra! {username} ist am {weekday} wieder im Büro! 🥳🎉",
"Wir haben eine großartige Nachricht: {username} wird am {weekday} ins Büro kommen! 👏🏼",
"Wir freuen uns, {username} an diesem {weekday} im Büro zu haben! 🙌🏼🎉",
"🎉 {username} wird am {weekday} im Büro sein! 🎉",
"Es ist großartig, dass {username} am {weekday} zurück im Büro ist! 🤗",
"Wir sind begeistert, {username} an diesem {weekday} im Büro zu begrüßen! 🎊",
"{username} wird am {weekday} ins Büro kommen - wir können es kaum erwarten! 🤩",
"Es ist ein Grund zum Feiern, denn {username} wird am {weekday} im Büro sein! 🎉👍🏼",
"Wir freuen uns, {username} am {weekday} wieder im Büro zu sehen! 🙌🏼",
"Willkommen zurück, {username}! Wir können es kaum erwarten, ihn am {weekday} im Büro zu sehen. 😁",
"Wir sind aufgeregt, {username} an diesem {weekday} im Büro zu haben! 🤗",

//kannst du weiter machen?

"🥳 {username} wird am {weekday} im Büro sein! 🥳",
"🎉 Großartig, dass {username} diesen {weekday} ins Büro kommt!",
"🎊 Wir können es kaum erwarten, {username} am {weekday} im Büro zu sehen!",
"💪 Super, dass {username} diesen {weekday} wieder im Büro ist!",
"🙌 Endlich ist {username} am {weekday} wieder zurück im Büro!",
"🤗 Wir freuen uns auf {username}'s Ankunft am {weekday} im Büro!",
"👍 {username} kommt am {weekday} ins Büro - das wird ein guter Tag!",
"👏 Ein Hoch auf {username}, der am {weekday} ins Büro kommt!",
"😍 Wir sind begeistert, dass {username} am {weekday} im Büro ist!",
"🥳 Hooray! {username} kommt diesen {weekday} ins Büro!",
"🤩 Wir können es kaum erwarten, {username} diesen {weekday} im Büro zu begrüßen!",

// announce auf 50 verschiedene arten, dass {username} am {weekday} ins büro kommt und feier das ab. du kannst auch emojis nutzen. ersetze {username} durch {username} und {weekday} durch {weekday}. Variiere auch die emojiis und die anzahl der emojis die du nutzt. die sprache ist deutsch. schreibe sehr formell

"Wir haben die erfreuliche Nachricht erhalten, dass {username} am {weekday} ins Büro kommen wird!",
"Es freut uns sehr mitteilen zu können, dass {username} am {weekday} den Weg ins Büro finden wird.",
"Mit großer Freude möchten wir bekanntgeben, dass {username} uns am {weekday} im Büro besuchen wird.",
"Es ist uns eine große Freude, zu verkünden, dass {username} am {weekday} anwesend sein wird.",
"Wir sind erfreut, Ihnen mitzuteilen, dass {username} am {weekday} im Büro anwesend sein wird.",
"Wir möchten mit großer Freude bekanntgeben, dass {username} am {weekday} im Büro erscheinen wird.",
"Es ist uns eine große Freude, mitteilen zu können, dass {username} am {weekday} im Büro anwesend sein wird.",
"Wir sind sehr erfreut, Ihnen mitteilen zu dürfen, dass {username} am {weekday} im Büro sein wird.",
"Es ist uns eine große Freude, bekanntgeben zu können, dass {username} am {weekday} im Büro anwesend sein wird.",
"Wir freuen uns sehr, Ihnen mitzuteilen, dass {username} am {weekday} im Büro anwesend sein wird.",
"Wir sind erfreut, Ihnen mitteilen zu können, dass {username} am {weekday} im Büro erscheinen wird.",
"Wir möchten mit großer Freude bekanntgeben, dass {username} am {weekday} anwesend sein wird.",
"Es ist uns eine große Freude, mitteilen zu können, dass {username} am {weekday} im Büro anwesend sein wird.",
"Wir sind sehr erfreut, Ihnen mitteilen zu dürfen, dass {username} am {weekday} anwesend sein wird.",

// announce auf 50 verschiedene arten, dass {username} am {weekday} ins büro kommt und feier das ab. du kannst auch emojis nutzen. ersetze {username} durch {username} und {weekday} durch {weekday}. Variiere auch die emojiis und die anzahl der emojis die du nutzt. die sprache ist deutsch. schreibe wie marketing

"Wir freuen uns sehr, Ihnen mitteilen zu können, dass {username} diesen {weekday} im Büro sein wird! 🎉",
"Erleben Sie mit uns die Freude und Begeisterung, dass {username} diesen {weekday} im Büro sein wird! 🎊",
"Machen Sie sich bereit für eine produktive und erfolgreiche Woche mit {username} im Büro am {weekday}! 🚀",
"Ein herzliches Willkommen an {username}, der uns diesen {weekday} im Büro besuchen wird! 🤗",
"Es ist uns eine Freude, Ihnen mitteilen zu können, dass {username} diesen {weekday} im Büro sein wird. Lassen Sie uns diese großartige Neuigkeit feiern! 🥳",
"Wir sind begeistert, Ihnen mitteilen zu können, dass {username} diesen {weekday} im Büro sein wird. Lassen Sie uns zusammenarbeiten und großartige Dinge erreichen! 💪",
"Feiern Sie mit uns, dass {username} diesen {weekday} im Büro sein wird und wir gemeinsam an innovativen Lösungen arbeiten werden! 🎉🚀",
"Wir sind sehr glücklich darüber, dass {username} diesen {weekday} im Büro sein wird und wir gemeinsam erfolgreich sein werden! 💪",
"Herzlich Willkommen {username}! Wir freuen uns darauf, mit Ihnen diesen {weekday} im Büro zu arbeiten und großartige Ergebnisse zu erzielen! 👨‍💼💼",
"Wir sind begeistert, Ihnen mitteilen zu können, dass {username} diesen {weekday} im Büro sein wird. Lassen Sie uns zusammenkommen und großartige Dinge erreichen! 💼🤝🚀",
"Wir freuen uns darauf, mit {username} diesen {weekday} im Büro zusammenzuarbeiten und gemeinsam erfolgreich zu sein!",

// announce auf 50 verschiedene arten, dass {username} am {weekday} ins büro kommt und feier das ab. du kannst auch emojis nutzen. ersetze {username} durch {username} und {weekday} durch {weekday}. Variiere auch die emojiis und die anzahl der emojis die du nutzt. die sprache ist deutsch. nutze kleinkindersprache

"Hach, schaut mal wer am {weekday} ins Büro kommt! Unser {username}! 🎉👏",
"Heute kommt unser {username} ins Büro! Juchu! 🥳",
"Yippiiiieee! Unser {username} kommt am {weekday} ins Büro! 🎉🎊",
"Wir feiern unseren {username}, denn er kommt am {weekday} ins Büro! 🎉🎉",
"Huiii, {weekday} ist ein besonderer Tag! Unser {username} kommt ins Büro! 🤩🎉",
"Ein ganz besonderer Gast kommt {weekday} ins Büro: unser {username}! 🥳🎊",
"Hey, hey, unser {username} kommt {weekday} ins Büro! Das ist super cool! 😎👍",
"Juhuuuu, unser {username} kommt ins Büro! Das wird ein Spaß! 🎉🎉",
"Hoppala, wer kommt denn da an {weekday}? Unser {username}! 🤗🎉",
"Guckt mal, unser {username} kommt {weekday} ins Büro! Lasst uns feiern! 🎉🎉",
"Hurra, hurra, unser {username} kommt ins Büro! Das wird ein toller Tag! 🥳🎊",
"Ein besonderer Besuch kommt {weekday} ins Büro: Unser {username}! 🤩🎉",
"Wie aufregend, unser {username} kommt {weekday} ins Büro! Lasst uns feiern! 🎉🎉",
"Juhu, {weekday} ist ein ganz besonderer Tag, denn unser {username} kommt ins Büro! 🥳🎊",
"Aufgepasst, unser {username} kommt {weekday} ins Büro! Wir freuen uns riesig! 🤗🎉",
"Hey, hey, unser {username} ist {weekday} im Büro! Das wird ein toller Tag! 🎉👍",
"Wir haben {weekday} einen ganz besonderen Gast im Büro: unser {username}! 🤩🎉",
"Heute wird ein ganz besonderer Tag, denn unser {username} kommt ins Büro! 🎊🎉",

// announce auf 50 verschiedene arten, dass {username} am {weekday} ins büro kommt und feier das ab. du kannst auch emojis nutzen. ersetze {username} durch {username} und {weekday} durch {weekday}. Variiere auch die emojiis und die anzahl der emojis die du nutzt. die sprache ist deutsch. nutze pastorsprache

"Liebe Gemeinde,\n\nes ist mir eine große Freude, euch mitteilen zu dürfen, dass unser Bruder {username} an diesem kommenden {weekday} seinen Weg in unser Büro finden wird. Lasst uns gemeinsam feiern und ihm einen warmen Empfang bereiten.\n\nLasst uns dankbar sein für diese Gelegenheit, zusammenzukommen und eine weitere Woche im Namen der Arbeit zu beginnen. Möge dieser Tag mit der Präsenz unseres Bruders {username} noch besonderer werden.\n\nLasst uns beten für eine sichere und reibungslose Anreise und für einen produktiven Tag voller Erfolge. Wir glauben daran, dass jeder von uns auf seine Weise zum Erfolg unseres Unternehmens beitragen kann, und so bitten wir den Herrn, dass er uns die Weisheit und Stärke gibt, dies zu erreichen.\n\nLasst uns in Dankbarkeit und Vorfreude zusammenkommen, um unseren Bruder {username} willkommen zu heißen und gemeinsam einen fruchtbaren Tag zu verbringen. Amen. 🙏",

// stell dir vor du bist ein spieß bei der bundeswehr

"Achtung, Achtung! {username} wird am {weekday} im Büro erwartet.",
"{username} wird am {weekday} im Büro anwesend sein.",
"Am {weekday} wird {username} den Weg ins Büro finden.",
"Es wird erwartet, dass {username} am {weekday} ins Büro kommt.",
"{username} hat für {weekday} einen Arbeitsplatz im Büro reserviert.",
"Wir begrüßen {username} am {weekday} im Büro.",
"Am {weekday} wird {username} Teil des Büroteams sein.",
"{username} wird am {weekday} im Büro erwartet, bereit für die Arbeit.",
"Bitte begrüßen Sie {username} am {weekday} im Büro.",
"Wir freuen uns darauf, {username} am {weekday} im Büro zu sehen.",
"Am {weekday} wird {username} im Büro erscheinen.",
"Wir erwarten {username} am {weekday} im Büro.",
"{username} wird am {weekday} anwesend sein.",
"Das Büro freut sich auf den Besuch von {username} am {weekday}.",
"Am {weekday} wird {username} im Büro arbeiten.",
"Wir begrüßen {username} im Büro am {weekday}.",
"{username} wird am {weekday} im Büro erwartet, um seine Arbeit zu erledigen.",
"Das Büro wird am {weekday} Besuch von {username} erhalten.",
"Bitte heißen Sie {username} am {weekday} im Büro willkommen.",
"Wir freuen uns darauf, {username} am {weekday} im Büro zu begrüßen.",
"{username} wird am {weekday} im Büro erscheinen, um seine Aufgaben zu erledigen.",
"Wir erwarten {username} am {weekday} im Büro und freuen uns auf seine Arbeit.",
"Am {weekday} wird {username} im Büro präsent sein.",
"Wir heißen {username} am {weekday} im Büro willkommen.",

// stell dir vor du bist ein marktschreier

"Na, höret, höret! Am {weekday} ist es soweit! Unser {username} wird den Weg ins Büro antreten! Kommet zahlreich und feiret mit uns!",
"🎉🎉🎉",
"Aufgepasst, aufgepasst! Am {weekday} ist der große Tag, an dem {username} wieder im Büro erscheint! Die Freude ist unbeschreiblich, also seid dabei und lasst uns gemeinsam feiern!",
"🥳🥳🥳",
"Willkommen, willkommen, willkommen! Am {weekday} wird unser {username} das Büro wieder mit seiner Anwesenheit beehren! Seid dabei, wenn wir diesen besonderen Moment feiern!",
"🎊🎊🎊",
"Trommelwirbel, Trompetenfanfare! {username} wird am {weekday} ins Büro zurückkehren und wir sind mehr als bereit, das gebührend zu feiern! Feiert mit uns!",
"🎺🥁🎺",
"Hört, hört! Am {weekday} wird unser {username} ins Büro zurückkehren und wir können das Wiedersehen es kaum erwarten! Kommt alle und lasst uns feiern!",
"🎉🎉🎉",
"Oh meine Damen und Herren, das Warten hat ein Ende! Unser {username} wird am {weekday} das Büro betreten und wir sind bereit für einen gebührenden Empfang! Feiert mit uns!",
"🎊🎊🎊",
"Seid bereit, meine Freunde! Am {weekday} kehrt {username} ins Büro zurück und wir können es kaum erwarten! Lasst uns gemeinsam feiern und den Moment genießen!",
"🥳🥳🥳",
"Ladies and Gentlemen, am {weekday} ist es soweit! {username} wird zurück ins Büro kehren und wir freuen uns auf das Wiedersehen!",
"🎉🎉🎉",
"Tatatataaa! Am {weekday} wird {username} das Büro wieder mit Anwesenheit beehren! Kommt alle und feiert mit uns diesen besonderen Moment!",
"🎊🎊🎊",
"Macht euch bereit, meine Lieben! Am {weekday} wird {username} ins Büro zurückkehren und wir freuen uns auf ein Wiedersehen! Lasst uns zusammen feiern und die Freude teilen!",
"🥳🥳🥳",
"Guten Tag, liebe Kollegen und Kolleginnen! Am {weekday} wird {username} ins Büro zurückkehren und wir sind bereit für einen gebührenden Empfant! Kommt alle und feiert mit uns!",
"🎉🎉🎉",
"Hört, hört! Am {weekday} wird {username} ins Büro zurückkehren und wir können es kaum erwarten! Seid dabei und lasst uns gemeinsam feiern!",
"🎊🎊🎊",
"Hallo, Hallo, Hallo! Am {weekday} wird {username} das Büro wieder mit seiner Anwesenheit beehren und wir sind mehr als bereit! Kommt alle und feiert mit uns!",
"🥳"];
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

    tt.add_template("", &random_sentence).unwrap();

    match tt.render("", &data) {
        Ok(s) => s,
        Err(e) => {
            error!("error rendering '{random_sentence}': {e}");
            "DANIEL HAT ES VERBOCKT ... SOFORT ANSCHREIEN BITTE".to_string()
        }
    }
}
