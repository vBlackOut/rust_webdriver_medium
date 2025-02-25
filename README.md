# 🦀 Rust WebDriver - Medium.com RSS Feed Scraper

Ce projet utilise [Thirtyfour](https://github.com/stevepryde/thirtyfour) pour automatiser la recherche de profils sur **Medium.com** et extraire leurs **flux RSS**.

---
## Installation et Exécution

### Dépendances
```
- Thirtyfour : >= 0.32  
- ChromeDriver : 133.0.6943.98 (ou version compatible)  
```

### **Cloner le projet**
```sh
git clone https://github.com/vBlackOut/rust_webdriver_medium.git
cd rust_webdriver_medium
```

### Installer les dépendances
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

### Télécharger et lancer chromedriver
```./chromedriver```

### remplacer le port et le mettre dans src/main.rs la ligne.
```let driver = WebDriver::new("http://localhost:34997", caps).await?;```

### aprés lancement de chromedriver
```
Starting ChromeDriver 133.0.6943.126 (cffa127ce7b6be72885391527c15b452056a2e81-refs/branch-heads/6943@{#1570}) on port 0
Only local connections are allowed.
Please see https://chromedriver.chromium.org/security-considerations for suggestions on keeping ChromeDriver safe.
ChromeDriver was started successfully on port 32935.
```

### Compiler et exécuter le projet
```sh
cargo build
cargo run
```  


