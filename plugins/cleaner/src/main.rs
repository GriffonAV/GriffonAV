use griffon_cleaner::{
    ExecutionContext, CleanerConfig, Profile,
    run_modules, default_modules, GlobalReport,
};

fn human_readable(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

fn print_cache_report(global: &GlobalReport) {
    // On récupère le report du module cache
    let cache_report = match global.per_module.get("cache") {
        Some(r) => r,
        None => {
            println!("Aucun rapport pour le module 'cache'.");
            return;
        }
    };

    println!("=== CacheCleaner Report ===");
    println!("Dry-run : {}", global.dry_run);
    println!("Total fichiers : {}", cache_report.files_touched);
    println!("Total libéré : {}", human_readable(cache_report.bytes_freed));

    // Par dossier racine
    if !cache_report.per_root_path.is_empty() {
        println!("\nPar dossier :");
        let mut entries: Vec<_> = cache_report.per_root_path.iter().collect();
        entries.sort_by_key(|(path, _)| *path);

        for (path, stats) in entries {
            println!(
                "- {} : {} fichiers, {}",
                path,
                stats.files_touched,
                human_readable(stats.bytes_freed),
            );
        }
    }

    // Par type de fichier
    if !cache_report.per_file_type.is_empty() {
        println!("\nPar type :");
        let mut entries: Vec<_> = cache_report.per_file_type.iter().collect();
        entries.sort_by_key(|(typ, _)| *typ);

        for (typ, stats) in entries {
            println!(
                "- {} : {} fichiers, {}",
                typ,
                stats.files_touched,
                human_readable(stats.bytes_freed),
            );
        }
    }

    // Permissions
    if cache_report.permission_denied > 0 {
        println!(
            "\nPermission denied : {} fichiers (lancer en root pour tout nettoyer)",
            cache_report.permission_denied
        );
    }

    // Warnings éventuels
    if !cache_report.warnings.is_empty() {
        println!("\nWarnings :");
        for w in &cache_report.warnings {
            println!("- {}", w);
        }
    }

    if !cache_report.errors.is_empty() {
        println!("\nErreurs :");
        for e in &cache_report.errors {
            println!("- {}", e);
        }
    }

    println!("===========================");
    println!("Module Used:");
}

fn whats_enabled_modules(cfg: &CleanerConfig) -> Vec<&'static str> {
    let mut res = Vec::new();

    if cfg.enable_system_cache {
        res.push("System");
    }
    if cfg.enable_user_cache {
        res.push("User");
    }
    if cfg.enable_browser_cache {
        res.push("Browser");
    }
    if cfg.enable_dev_cache {
        res.push("DevTools");
    }
    if cfg.enable_package_cache {
        res.push("PackageManager");
    }
    if cfg.enable_desktop_cache {
        res.push("DesktopEnv");
    }

    res
}

fn main() {
    let config = CleanerConfig {
        profile: Profile::Safe,
        max_log_retention_days: 30,
        max_log_size_gb: 2.0,
        min_bigfile_size_mb: 100,

        enable_system_cache: true,
        enable_user_cache: true,
        enable_browser_cache: false,     // on évite de casser les sessions de navigation des users
        enable_dev_cache: true,
        enable_package_cache: true,
        enable_desktop_cache: true,
    };

    let ctx = ExecutionContext {
        config,
        dry_run: true, // garde true pour tester sinon tu vas vraiment supprimer des fichiers :)
        root_paths: vec!["/".into()],
    };

    let modules = default_modules();

    match run_modules(&ctx, &modules) {
        Ok(report) => {
            print_cache_report(&report);
            let enabled = whats_enabled_modules(&ctx.config);
            println!("Enabled Cache Modules: {:?}", enabled);
        }
        Err(e) => {
            eprintln!("Erreur lors de l'exécution du cleaner : {:?}", e);
        }
    }
}