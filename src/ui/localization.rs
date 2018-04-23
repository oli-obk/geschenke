use fluent::MessageContext;
use fluent::types::FluentValue;
use std::collections::HashMap;

let mut ctx_eng = MessageContext::new(&["en-US"]);

// TODO: read content of add_messages in json files
ctx_eng.add_messages(
    "
hello-world = Hello, world!

## general stuff
you-big = You
you-small = you

## app name
-app-name = Geschenkeplanungsapp

## add friend
already-friends = You are already friends
friend-self-hugging = You cannot befriend yourself
added-friend = Added new friend
unregistered-friend = Could not add unregistered friend

## delete friend
deleted-friend = Deleted friend
friend-not-deleted = Could not delete friend

## wishlist
delete = Delete
edit = Edit
create-present = Create new present
recipient = The present is for { $recipient }
description = Description
save = Save

## present addition/editing/removal
added-present = Added new present
present-exists = A present with the same name already exists
not-adding-without-description = Cannot add a present without a description
present-saved = Present saved
present-deleted = Deleted present

## Logging in/out
login-successful = Successfully logged in.
logout-successful = Successfully logged out.
wrong-key = Wrong or old login key
wrong-mail-or-password = Unknown email address or wrong password
forgotten-login = Forgotten Login

## Login interface
login = Login
register = Register
mail = E-Mail
password = Password
key = Key

## info texts
info-login = To login click on the login link that is in every email that you get.
info-enter-correct-email = Make sure to enter the correct email address below, no emails will be sent if you enter a wrong email address.
info-resend-login-mail = Resend login email

## emails
login-mail =
    Someone (probably you) has requested a new login link for https://geschenke.oli-obk.de .\r\n\
    \r\n\
    Click the following link to login:\r\n\
    https://geschenke.oli-obk.de/account/login_form_key?key={ $autologin } \r\n\
    \r\n\
    Your friendly neighborhood Geschenke-Bot\r\n\
    \r\n\
    \r\n\
    If it was not you, no damage has been done, just use the new login link\r\n\
    Your account is still perfectly safe\r\n
registration-mail =
    Someone (probably you) has created an account for {email_address} at https://geschenke.oli-obk.de .\r\n\
    \r\n\
    Click the following link to login:\r\n\
    https://geschenke.oli-obk.de/account/login_form_key?key={autologin} \r\n\
    \r\n\
    Your friendly neighborhood Geschenke-Bot\r\n\
    \r\n\
    \r\n\
    If it was not you, visit\r\n\
    https://geschenke.oli-obk.de/nuke/{autologin}.\r\n\
    to remove your email address from our database\r\n

## dashboard (logged_in)
home = Home
intro = Hello, { $name }.
wishlist = Your wishlist
friends = Friends
add-friend = Add friend
logout = Logout

## registration
email-sent = An email with login instructions has been sent to { $email }
email-alread-registered = This email is already registered
email-invalid = That's not an email address
email-not-sent = Please contact an admin, emails could not be sent
"
    );

let mut ctx_de = MessageContext::new(&["de-DE"]);
ctx_de.add_messages(
    "
hello-world = Hallo, Welt!

## general stuff
you-big =
    {
        [duzen] Du
       *[siezen] Sie
    }
you-small =
    {
        [duzen] du
       *[siezen] Sie
    }
yourself =
    {
        [duzen] dich
       *[siezen] sich
    }

## app name
-app-name = Geschenkeplanungsapp

## add friend
already-friends =
    {
        [duzen] Ihr seid bereits befreundet
       *[siezen] Sie sind bereits befreundet
    }
friend-self-hugging =
    {
        [duzen] Du kannst nicht mit dir selbst befreundet sein
       *[siezen] Sie können nicht mit sich selbst befreundet sein
    }
added-friend = Neuer Freund hinzugefügt
unregistered-friend = Nicht registrierter Freund konnte nicht hinzugefügt werden

## delete friend
deleted-friend = Freund gelöscht
friend-not-deleted = Freund konnte nicht gelöscht werden

## wishlist
delete = Löschen
edit = Bearbeiten
create-present = Neues Geschenk erstellen
recipient = Das Geschenk ist für { $recipient }
description = Beschreibung
save = Speichern

# present addition/editing/removal
added-present = Neues Geschenk hinzugefügt
present-exists = Ein Geschenk mit dem gleichen Namen existiert bereits
not-adding-without-description = Ein Geschenk kann nicht ohne Beschreibung hinzugefügt werden
present-saved = Geschenk gespeichert
present-deleted = Geschenk gelöscht

## Logging in/out
login-successful = Erfolgreich eingeloggt.
logout-successful = Erfolgreich ausgeloggt.
wrong-key = Falscher oder veralteter Loginschlüssel
wrong-mail-or-password = Unbekannte Emailadresse oder falsches Passwort
forgotten-login = Login vergessen

## Login interface
login = Login
register = Registrieren
mail = E-Mail
password = Passwort
key = Schlüssel

## info texts
click =
    {
        [duzen] klick
       *[siezen] klicken Sie
    }
receive =
    {
        [duzen] empfängst
       *[siezen] empfangen
    }
# formality = [duzen|siezen]
info-login = Um { yourself[ $formality ] } einzuloggen, { click[ $formality ] } auf den Link der
             in jeder { mail } ist, die { you-small } von uns { receive[ $formality ] }.
#

info-login =
    {
        [duzen] Um sich einzuloggen, klick auf den Link der in jeder { mail } ist, die du von uns empfängst.
       *[siezen] Um sich einzuloggen, klicken Sie auf den Link der in jeder { mail } ist, die Sie von uns empfangen.
    }
#

info-enter-correct-email =
    {
        [duzen] Achte darauf unten die richtige { mail }adresse anzugeben. Wenn du die falsche { mail }adresse angeben, werden keine { mail }s verschickt
       *[siezen] Achten Sie darauf unten die richtige { mail }adresse anzugeben. Wenn Sie die falsche { mail }adresse angeben, werden keine { mail }s verschickt
    }
info-resend-login-mail = Login { mail } erneut senden

## emails
login-mail =
    Someone (probably you) has requested a new login link for https://geschenke.oli-obk.de .\r\n\
    \r\n\
    Click the following link to login:\r\n\
    https://geschenke.oli-obk.de/account/login_form_key?key={ $autologin } \r\n\
    \r\n\
    Your friendly neighborhood Geschenke-Bot\r\n\
    \r\n\
    \r\n\
    If it was not you, no damage has been done, just use the new login link\r\n\
    Your account is still perfectly safe\r\n
registration-mail =
    Someone (probably you) has created an account for {email_address} at https://geschenke.oli-obk.de .\r\n\
    \r\n\
    Click the following link to login:\r\n\
    https://geschenke.oli-obk.de/account/login_form_key?key={ $autologin } \r\n\
    \r\n\
    Your friendly neighborhood Geschenke-Bot\r\n\
    \r\n\
    \r\n\
    If it was not you, visit\r\n\
    https://geschenke.oli-obk.de/nuke/{autologin}.\r\n\
    to remove your email address from our database\r\n

## dashboard (logged_in)
home = Home
intro = Hello, { $name }.
wishlist = Your wishlist
friends = Friends
add-friend = Add friend
logout = Logout

## registration
email-sent = An email with login instructions has been sent to { $email }
email-alread-registered = This email is already registered
email-invalid = That's not an email address
email-not-sent = Please contact an admin, emails could not be sent

"
    );

let msg = ctx_eng.get_message("hello-world").unwrap();
let value = ctx_eng.format(msg, None).unwrap();

assert_eq!(value, "Hello, world!");

let mut args = HashMap::new();
args.insert("name", FluentValue::from("John"));

let msg = ctx_eng.get_message("intro").unwrap();
let value = ctx_eng.format(msg, Some(&args)).unwrap();

assert_eq!(value, "Welcome, John.");
