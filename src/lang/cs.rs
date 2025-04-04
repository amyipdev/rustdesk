lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stav"),
        ("Your Desktop", "Vaše plocha"),
        ("desk_tip", "Pomocí tohoto identifikátoru a hesla můžete přistupovat ke své ploše."),
        ("Password", "Heslo"),
        ("Ready", "Připraveno"),
        ("Established", "Navázáno"),
        ("connecting_status", "Připojování se k Rusdesk síti…"),
        ("Enable Service", "Povolit službu"),
        ("Start Service", "Spustit službu"),
        ("Service is running", "Služba je spuštěná"),
        ("Service is not running", "Služba není spuštěná"),
        ("not_ready_status", "Nepřipraveno. Zkontrolujte své připojení."),
        ("Control Remote Desktop", "Ovládat vzdálenou plochu"),
        ("Transfer File", "Přenést soubor"),
        ("Connect", "Připojit"),
        ("Recent Sessions", "Nedávné relace"),
        ("Address Book", "Adresář kontaktů"),
        ("Confirmation", "Potvrzení"),
        ("TCP Tunneling", "TCP tunelování"),
        ("Remove", "Odebrat"),
        ("Refresh random password", "Vytvořit nové náhodné heslo"),
        ("Set your own password", "Nastavte si své vlastní heslo"),
        ("Enable Keyboard/Mouse", "Povolit klávesnici/myš"),
        ("Enable Clipboard", "Povolit schránku"),
        ("Enable File Transfer", "Povolit přenos souborů"),
        ("Enable TCP Tunneling", "Povolit TCP tunelování"),
        ("IP Whitelisting", "Povolování pouze z daných IP adres)"),
        ("ID/Relay Server", "Identifikátor / předávací (relay) server"),
        ("Import Server Config", "Importovat konfiguraci serveru"),
        ("Export Server Config", ""),
        ("Import server configuration successfully", "Konfigurace serveru úspěšně importována"),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", "Neplatná konfigurace serveru"),
        ("Clipboard is empty", "Schránka je prázdná"),
        ("Stop service", "Zastavit službu"),
        ("Change ID", "Změnit identifikátor"),
        ("Website", "Webové stránky"),
        ("About", "O aplikaci"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Ztlumit"),
        ("Audio Input", "Vstup zvuku"),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive Bitrate", ""),
        ("ID Server", "Server pro identif."),
        ("Relay Server", "Předávací (relay) server"),
        ("API Server", "Server s API rozhraním"),
        ("invalid_http", "Je třeba, aby začínalo na http:// nebo https://"),
        ("Invalid IP", "Neplatná IP adresa"),
        ("id_change_tip", "Použít je mozné pouze znaky a-z, A-Z, 0-9 a _ (podtržítko). Dále je třeba aby začínalo na písmeno a-z, A-Z. Délka mezi 6 a 16 znaky."),
        ("Invalid format", "Neplatný formát"),
        ("server_not_support", "Server zatím nepodporuje"),
        ("Not available", "Není k dispozici"),
        ("Too frequent", "Příliš časté"),
        ("Cancel", "Storno"),
        ("Skip", "Přeskočit"),
        ("Close", "Zavřít"),
        ("Retry", "Zkusit znovu"),
        ("OK", "OK"),
        ("Password Required", "Vyžadováno heslo"),
        ("Please enter your password", "Zadejte své heslo"),
        ("Remember password", "Zapamatovat heslo"),
        ("Wrong Password", "Nesprávné heslo"),
        ("Do you want to enter again?", "Chcete se znovu připojit?"),
        ("Connection Error", "Chyba spojení"),
        ("Error", "Chyba"),
        ("Reset by the peer", "Resetováno protějškem"),
        ("Connecting...", "Připojování…"),
        ("Connection in progress. Please wait.", "Probíhá připojování – vyčkejte."),
        ("Please try 1 minute later", "Zkuste to až za minutu či déle"),
        ("Login Error", "Chyba přihlášení se"),
        ("Successful", "Úspěšné"),
        ("Connected, waiting for image...", "Připojeno, čeká se na obraz…"),
        ("Name", "Název"),
        ("Type", "Typ"),
        ("Modified", "Změněno"),
        ("Size", "Velikost"),
        ("Show Hidden Files", "Zobrazit skryté soubory"),
        ("Receive", "Přijmout"),
        ("Send", "Odeslat"),
        ("Refresh File", "Znovu načíst soubor"),
        ("Local", "Místní"),
        ("Remote", "Vzdálené"),
        ("Remote Computer", "Vzdálený počítač"),
        ("Local Computer", "Místní počítač"),
        ("Confirm Delete", "Potvrdit smazání"),
        ("Delete", "Smazat"),
        ("Properties", "Vlastnosti"),
        ("Multi Select", "Vícenásobný výběr"),
        ("Select All", ""),
        ("Unselect All", ""),
        ("Empty Directory", "Prázdná složka"),
        ("Not an empty directory", "Neprázdná složka"),
        ("Are you sure you want to delete this file?", "Opravdu chcete tento soubor vymazat?"),
        ("Are you sure you want to delete this empty directory?", "Opravdu chcete tuto prázdnou složku smazat?"),
        ("Are you sure you want to delete the file of this directory?", "Opravdu chcete vymazat soubor, pocházející z této složky?"),
        ("Do this for all conflicts", "Naložit takto se všemi konflikty"),
        ("This is irreversible!", "Toto nelze vzít zpět"),
        ("Deleting", "Mazání"),
        ("files", "soubory"),
        ("Waiting", "Čeká se"),
        ("Finished", "Dokončeno"),
        ("Speed", "Rychlost"),
        ("Custom Image Quality", "Uživatelsky určená kvalita obrazu"),
        ("Privacy mode", "Režim soukromí"),
        ("Block user input", "Blokovat vstupní zařízení uživatele"),
        ("Unblock user input", "Odblokovat vstupní zařízení uživatele"),
        ("Adjust Window", "Přizpůsobit velikost okna"),
        ("Original", "Původní"),
        ("Shrink", "Oříznout"),
        ("Stretch", "Roztáhnout"),
        ("Scrollbar", "Posuvník"),
        ("ScrollAuto", "Rolovať Auto"),
        ("Good image quality", "Dobrá kvalita obrazu"),
        ("Balanced", "Vyvážené"),
        ("Optimize reaction time", "Optimalizovat pro co nejnižší prodlevu odezvy"),
        ("Custom", ""),
        ("Show remote cursor", "Zobrazovat ukazatel myši z protějšku"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Vypnout schránku"),
        ("Lock after session end", "Po ukončení relace zamknout plochu"),
        ("Insert", "Vložit"),
        ("Insert Lock", "Vložit zámek"),
        ("Refresh", "Načíst znovu"),
        ("ID does not exist", "Takový identifikátor neexistuje"),
        ("Failed to connect to rendezvous server", "Nepodařil se připojit ke zprostředkovávajícímu serveru"),
        ("Please try later", "Zkuste to později"),
        ("Remote desktop is offline", "Vzdálená plocha není připojená ke službě"),
        ("Key mismatch", "Neshoda klíčů"),
        ("Timeout", "Překročen časový limit pro navázání spojení"),
        ("Failed to connect to relay server", "Nepodařilo se připojit k předávacímu (relay) serveru"),
        ("Failed to connect via rendezvous server", "Nepodařilo se připojit prostřednictvím zprostředkovávajícího serveru"),
        ("Failed to connect via relay server", "Nepodařilo se připojit prostřednictvím předávacímu (relay) serveru"),
        ("Failed to make direct connection to remote desktop", "Nepodařilo s navázat přímé připojení ke vzdálené ploše"),
        ("Set Password", "Nastavit heslo"),
        ("OS Password", "Heslo do operačního systému"),
        ("install_tip", "Kvůli řízení oprávnění v systému (UAC), RustDesk v některých případech na protějšku nefunguje správně. Abyste se UAC vyhnuli, klikněte na níže uvedené tlačítko a nainstalujte tak RustDesk do systému."),
        ("Click to upgrade", "Aktualizaci nainstalujete kliknutím"),
        ("Click to download", "Stáhnete si kliknutím"),
        ("Click to update", "Znovu načtete kliknutím"),
        ("Configure", "Nastavit"),
        ("config_acc", "Aby bylo možné na dálku ovládat vaši plochu, je třeba aplikaci RustDesk udělit oprávnění pro „Zpřístupnění pro hendikepované“."),
        ("config_screen", "Aby bylo možné přistupovat k vaší ploše na dálku, je třeba aplikaci RustDesk udělit oprávněí pro „Nahrávání obsahu obrazovky“."),
        ("Installing ...", "Instaluje se…"),
        ("Install", "Nainstalovat"),
        ("Installation", "Instalace"),
        ("Installation Path", "Popis umístění instalace"),
        ("Create start menu shortcuts", "Vytvořit zástupce v nabídce Start"),
        ("Create desktop icon", "Vytvořit ikonu na ploše"),
        ("agreement_tip", "Spuštěním instalace přijímáte licenční ujednání."),
        ("Accept and Install", "Přijmout a nainstalovat"),
        ("End-user license agreement", "Licencenční ujednání s koncovým uživatelem"),
        ("Generating ...", "Vytváření…"),
        ("Your installation is lower version.", "Máte nainstalovanou starší verzi"),
        ("not_close_tcp_tip", "Po dobu, po kterou tunel potřebujete, nezavírejte toto okno"),
        ("Listening ...", "Očekávní spojení…"),
        ("Remote Host", "Vzdálený stroj"),
        ("Remote Port", "Port na protějšku"),
        ("Action", "Akce"),
        ("Add", "Přidat"),
        ("Local Port", "Místní port"),
        ("Local Address", ""),
        ("Change Local Port", ""),
        ("setup_server_tip", "Rychlejší připojení získáte vytvořením si svého vlastního serveru"),
        ("Too short, at least 6 characters.", "Příliš krátké – alespoň 6 znaků."),
        ("The confirmation is not identical.", "Kontrolní zadání se neshoduje."),
        ("Permissions", "Oprávnění"),
        ("Accept", "Přijmout"),
        ("Dismiss", "Zahodit"),
        ("Disconnect", "Odpojit"),
        ("Allow using keyboard and mouse", "Umožnit ovládání mé klávesnice a myši"),
        ("Allow using clipboard", "Umožnit používání schránky"),
        ("Allow hearing sound", "Umožnit slyšet můj zvuk"),
        ("Allow file copy and paste", "Povolit kopírování a vkládání souborů"),
        ("Connected", "Připojeno"),
        ("Direct and encrypted connection", "Přímé a šifrované spojení"),
        ("Relayed and encrypted connection", "Předávané (relay) a šifrované spojení"),
        ("Direct and unencrypted connection", "Přímé a nešifrované spojení"),
        ("Relayed and unencrypted connection", "Předávané (relay) a nešifrované spojení"),
        ("Enter Remote ID", "Zadejte identifikátor protějšku"),
        ("Enter your password", "Zadejte své heslo"),
        ("Logging in...", "Přihlašování se…"),
        ("Enable RDP session sharing", "Zapnout sdílení relace RDP protokolu"),
        ("Auto Login", "Automatické přihlášení"),
        ("Enable Direct IP Access", "Zapnout přímý přístup na IP adresu"),
        ("Rename", "Přejmenovat"),
        ("Space", "Mezera"),
        ("Create Desktop Shortcut", "Vytvořit zástupce na ploše"),
        ("Change Path", "Změnit umístění"),
        ("Create Folder", "Vytvořit složku"),
        ("Please enter the folder name", "Zadejte název pro složku"),
        ("Fix it", "Opravit to"),
        ("Warning", "Upozornení"),
        ("Login screen using Wayland is not supported", "Přihlašovací obrazovka prostřednictvím Wayland není podporována"),
        ("Reboot required", "Je třeba restartovat"),
        ("Unsupported display server ", "Nepodporovaný zobrazovací server"),
        ("x11 expected", "očekávány x11"),
        ("Port", ""),
        ("Settings", "Nastavení"),
        ("Username", "Uživatelské jméno"),
        ("Invalid port", "Neplatné číslo portu"),
        ("Closed manually by the peer", "Ručně ukončeno protějškem"),
        ("Enable remote configuration modification", "Umožnit upravování nastavení vzdáleného"),
        ("Run without install", "Spustit bez instalování"),
        ("Always connected via relay", "Vždy spojováno prostřednictvím brány pro předávání (relay)"),
        ("Always connect via relay", "Vždy se spojovat prostřednictvím brány pro předávání (relay)"),
        ("whitelist_tip", "Přístup je umožněn pouze z IP adres, nacházejících se na seznamu povolených"),
        ("Login", "Přihlásit se"),
        ("Logout", "Odhlásit se"),
        ("Tags", "Štítky"),
        ("Search ID", "Hledat identifikátor"),
        ("Current Wayland display server is not supported", "Zobrazovací server Wayland zatím není podporován"),
        ("whitelist_sep", "Odělováno čárkou, středníkem, mezerou nebo koncem řádku"),
        ("Add ID", "Přidat identifikátor"),
        ("Add Tag", "Přidat štítek"),
        ("Unselect all tags", "Zrušit výběr všech štítků"),
        ("Network error", "Chyba sítě"),
        ("Username missed", "Chybí uživatelské jméno"),
        ("Password missed", "Chybí heslo"),
        ("Wrong credentials", "Nesprávné přihlašovací údaje"),
        ("Edit Tag", "Upravit štítek"),
        ("Unremember Password", "Přestat si heslo pamatovat"),
        ("Favorites", "Oblíbené"),
        ("Add to Favorites", "Přidat do oblíbených"),
        ("Remove from Favorites", "Odebrat z oblíbených"),
        ("Empty", "Prázdné"),
        ("Invalid folder name", "Neplatný název složky"),
        ("Socks5 Proxy", "Socks5 proxy"),
        ("Hostname", "Název stroje"),
        ("Discovered", "Objeveno"),
        ("install_daemon_tip", "Pokud má být spouštěno při startu systému, je třeba nainstalovat systémovou službu."),
        ("Remote ID", "Identif. protějšku"),
        ("Paste", "Vložit"),
        ("Paste here?", "Vložit sem?"),
        ("Are you sure to close the connection?", "Opravdu chcete spojení ukončit?"),
        ("Download new version", "Stáhnout si novou verzi"),
        ("Touch mode", "Režim dotyku"),
        ("Mouse mode", "Režim myši"),
        ("One-Finger Tap", "Klepnutí jedním prstem"),
        ("Left Mouse", "Levé tlačítko myši"),
        ("One-Long Tap", "Jedno dlouhé klepnutí"),
        ("Two-Finger Tap", "Klepnutí dvěma prsty"),
        ("Right Mouse", "Pravé tlačítko myši"),
        ("One-Finger Move", "Přesouvání jedním prstem"),
        ("Double Tap & Move", "Dvojité klepnutí a přesun"),
        ("Mouse Drag", "Přetažení myší"),
        ("Three-Finger vertically", "Třemi prsty svisle"),
        ("Mouse Wheel", "Kolečko myši"),
        ("Two-Finger Move", "Posun dvěma prsty"),
        ("Canvas Move", "Posun zobrazení"),
        ("Pinch to Zoom", "Přiblížíte roztažením dvěma prsty"),
        ("Canvas Zoom", "Přiblížení zobrazení"),
        ("Reset canvas", "Vrátit měřtko zobrazení na výchozí"),
        ("No permission of file transfer", "Žádné oprávnění přenosu souboru"),
        ("Note", "Poznámka"),
        ("Connection", "Připojení"),
        ("Share Screen", "Nasdílet obrazovku"),
        ("CLOSE", "ZAVŘÍT"),
        ("OPEN", "OTEVŘÍT"),
        ("Chat", "Chat"),
        ("Total", "Celkem"),
        ("items", "Položek"),
        ("Selected", "Vybráno"),
        ("Screen Capture", "Zachytávání obrazovky"),
        ("Input Control", "Ovládání vstupních zařízení"),
        ("Audio Capture", "Zachytávání zvuku"),
        ("File Connection", "Souborové spojení"),
        ("Screen Connection", "Spojení obrazovky"),
        ("Do you accept?", "Přijímáte?"),
        ("Open System Setting", "Otevřít nastavení systému"),
        ("How to get Android input permission?", "Jak v systému Android získat oprávnění pro vstupní zařízení?"),
        ("android_input_permission_tip1", "Aby vzdálené zařízení mohlo ovládat vaše Android zařízení prostřednictví myši či dotyků, je třeba povolit, aby RustDesk mohlo používat službu „Zpřístupnění hendikepovaným“."),
        ("android_input_permission_tip2", "Přejděte na následující stránku nastavení systému, najděte a přejděte do [Nainstalované služby] a zapněte službu [RustDesk vstup]."),
        ("android_new_connection_tip", "Obdržen nový požadavek na řízení zařízení, který chce ovládat vaše stávající zařízení."),
        ("android_service_will_start_tip", "Zapnutí „Zachytávání obsahu obrazovky“ automaticky spustí službu, což umožní ostatním zařízením žádat o připojení k vašemu zařízení."),
        ("android_stop_service_tip", "Zastavení služby automaticky ukončí veškerá navázaná spojení."),
        ("android_version_audio_tip", "Vámi nyní používaná verze systému Android nepodporuje zachytávání zvuku – přejděte na Android 10 nebo novější."),
        ("android_start_service_tip", "Službu pro sdílení obrazovky spustíte klepnutím na [Spustit službu] nebo UDĚLTE pověření pro [Zachytávání obsahu obrazovky]."),
        ("Account", ""),
        ("Overwrite", "Přepsat"),
        ("This file exists, skip or overwrite this file?", "Tento soubor existuje – přeskočit ho nebo přepsat?"),
        ("Quit", "Ukončit"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Nápověda"),
        ("Failed", "Nepodařilo se"),
        ("Succeeded", "Uspěl"),
        ("Someone turns on privacy mode, exit", "Někdo zapne režim soukromí, ukončete ho"),
        ("Unsupported", "Nepodporováno"),
        ("Peer denied", "Peer popřel"),
        ("Please install plugins", "Nainstalujte si prosím pluginy"),
        ("Peer exit", "Peer exit"),
        ("Failed to turn off", "Nepodařilo se vypnout"),
        ("Turned off", "Vypnutý"),
        ("In privacy mode", "v režimu soukromí"),
        ("Out privacy mode", "mimo režim soukromí"),
        ("Language", ""),
        ("Keep RustDesk background service", ""),
        ("Ignore Battery Optimizations", ""),
        ("android_open_battery_optimizations_tip", ""),
        ("Connection not allowed", ""),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Enable Remote Restart", ""),
        ("Allow remote restart", ""),
        ("Restart Remote Device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting Remote Device", ""),
        ("remote_restarting_tip", ""),
        ("Copied", ""),
        ("Exit Fullscreen", "Ukončete celou obrazovku"),
        ("Fullscreen", "Celá obrazovka"),
        ("Mobile Actions", "Mobilní akce"),
        ("Select Monitor", "Vyberte možnost Monitor"),
        ("Control Actions", "Ovládací akce"),
        ("Display Settings", "Nastavení obrazovky"),
        ("Ratio", "Poměr"),
        ("Image Quality", "Kvalita obrazu"),
        ("Scroll Style", "Štýl posúvania"),
        ("Show Menubar", "Zobrazit panel nabídek"),
        ("Hide Menubar", "skrýt panel nabídek"),
        ("Direct Connection", "Přímé spojení"),
        ("Relay Connection", "Připojení relé"),
        ("Secure Connection", "Zabezpečené připojení"),
        ("Insecure Connection", "Nezabezpečené připojení"),
        ("Scale original", "Měřítko původní"),
        ("Scale adaptive", "Měřítko adaptivní"),
        ("General", ""),
        ("Security", ""),
        ("Theme", ""),
        ("Dark Theme", ""),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", ""),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", ""),
        ("Enable Audio", ""),
        ("Unlock Network Settings", ""),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", ""),
        ("Deny remote access", ""),
        ("Use IP Whitelisting", ""),
        ("Network", ""),
        ("Enable RDP", ""),
        ("Pin menubar", "Připnout panel nabídek"),
        ("Unpin menubar", "Odepnout panel nabídek"),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable Recording Session", ""),
        ("Allow recording session", ""),
        ("Enable LAN Discovery", ""),
        ("Deny LAN Discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", ""),
        ("Full Access", ""),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland vyžaduje Ubuntu 21.04 nebo vyšší verzi."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland vyžaduje vyšší verzi linuxové distribuce. Zkuste prosím X11 desktop nebo změňte OS."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Vyberte prosím obrazovku, kterou chcete sdílet (Ovládejte na straně protějšku)."),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by the web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
    ].iter().cloned().collect();
}
