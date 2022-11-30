lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Desktopi juaj mund të aksesohet me këtë ID dhe fjalëkalim."),
        ("connecting_status", "Duke u lidhur me rrjetin RustDesk.")
        ("not_ready_status", "Jo gati.Ju lutem kontolloni lidhjen tuaj."),
        ("id_change_tip", "Lejohen Vetëm karkteret a-z,A-Z,0-9 dhe _(nënvizimet).Shkronja e parë duhet të jetë a-z, A-Z. Gjatesia midis 6 dhe 16."),
        ("install_tip", "Për shkak të UAC, Rustdesk nuk mund të punoj sic duhet si nje remote në distancë në disa raste. Për të shamngur UAC, ju lutem klikoni butonin më poshtë për të instaluar RustDesk në sistem."),
        ("config_acc", "Për të kontrolluar Desktopin tuaj nga distanca, duhet të jepni leje RustDesk \"Aksesueshmëri\"."),
        ("config_screen", "Për të aksesuar Desktopin tuaj nga distanca, duhet ti jepni lejet RustDesk \"Regjistrimin e ekranit\"."),
        ("agreement_tip", "Duke filluar instalimin, ju pranoni marrëveshjen e licencës"),
        ("not_close_tcp_tip", "Mos e mbyll këtë dritare ndërsa jeni duke  përdorur tunelin"),
        ("setup_server_tip", "Për lidhje më të shpejtë, ju lutemi konfiguroni serverin tuaj"),
        ("Auto Login", "Hyrja automatike (e vlefshme vetëm nëse vendosni \"Kyçja pas përfundimit të sesionit\")."),
        ("whitelist_tip", "Vetëm IP e listës së bardhë mund të më aksesoj."),
        ("whitelist_sep", "Të ndara me presje, pikëpresje, hapësira ose rresht të ri"),
        ("wrong credentials", "Emri i përdoruesit ose fjalëkalimi i gabuar"),
        ("invalid_http", "Duhet të fillojë me http:// ose https://"),
        ("install_daemon_tip", "Për të nisur në boot, duhet të instaloni shërbimin e sistemit"),
        ("android_input_permission_tip1", "Në mënyrë që një pajisje në distancë të kontrollojë pajisjen tuaj Android nëpërmjet mausit ose prekjes, duhet të lejoni RustDesk të përdorë shërbimin."),
        ("android_input_permission_tip2", "Ju lutemi shkoni në faqen tjetër të cilësimeve të sistemit, gjeni dhe shtypni [Shërbimet e Instaluara], aktivizoni shërbimin [RustDesk Input]"),
        ("android_new_connection_tip", "Është marrë një kërkesë e re kontrolli, e cila dëshiron të kontrollojë pajisjen tuaj aktuale."),
        ("android_service_will_start_tip", "Aktivizimi i \"Regjistrimi i ekranit\" do të nisë automatikisht shërbimin, duke lejuar pajisjet e tjera të kërkojnë një lidhje me pajisjen tuaj."),
        ("android_stop_service_tip", "Mbyllja e shërbimit do të mbyllë automatikisht të gjitha lidhjet e vendosura."),
        ("android_version_audio_tip", "Versioni aktual i Android nuk mbështet regjistrimin e audios, ju lutemi përmirësoni në Android 10 ose më të lartë."),
        ("android_start_service_tip", "Shtyp [Fillo Shërbimin] ose HAP lejen e [Kapjen e Ekranit] për të nisur shërbimin e ndarjes së ekranit."),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("doc_fix_wayland", "https://rustdesk.com/docs/en/manual/linux/#x11-required"),
        ("server_not_support", "Nuk suportohet akoma nga severi"),
        ("android_open_battery_optimizations_tip", "Nëse dëshironi ta çaktivizoni këtë veçori, ju lutemi shkoni te faqja tjetër e cilësimeve të aplikacionit RustDesk, gjeni dhe shtypni [Batteri], hiqni zgjedhjen [Te pakufizuara]"),
        ("remote_restarting_tip", "Pajisja në distancë po riniset, ju lutemi mbyllni këtë kuti mesazhi dhe lidheni përsëri me fjalëkalim të përhershëm pas një kohe"),
        ("Are you sure to close the connection?", "A jeni i sigurt për të mbyllur lidhjen?"),
        ("elevation_prompt", "Drejtimi i softuerit pa ngritjen e privilegjeve mund të shkaktojë probleme kur përdoruesit në distancë përdorin dritare të caktuara."),
        ("uac_warning", "Qasja e refuzuar përkohësisht për shkak të kërkesës për lartësi, ju lutemi prisni që përdoruesi në distancë të pranojë dialogun UAC. Për të shmangur këtë problem, rekomandohet instalimi i softuerit në pajisjen në distancë ose ekzekutimi i tij me privilegje administratori."),
        ("elevated_foreground_window_warning", "Përkohësisht është e pamundur për të përdorur mausin dhe tastierën, për shkak se dritarja aktuale e desktopit në distancë kërkon privilegj më të lartë për të vepruar,ju mund t'i kërkoni përdoruesit në distancë të minimizojë dritaren aktuale. Për të shmangur këtë problem, rekomandohet të instaloni softuerin në pajisjen në distancë ose ekzekutoni atë me privilegje administratori."),
        ("JumpLink", "Shiko"),
        ("Stop service", "Ndalo shërbimin"),
        ].iter().cloned().collect();
}
