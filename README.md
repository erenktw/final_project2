# 📄 Final Project  - Token Freeze Smart Contract

## 📚 Proje Açıklaması

**Final Project Pro**, Stellar Soroban platformu üzerinde geliştirilmiş bir token sözleşmesidir.
Kullanıcıların varlıklarını dondurup çözmesine izin verirken, güvenlik ve kullanıcı kontrolünü artırmayı amaçlar.
Ayrıca sistem, yapılan işlemler için **event (olay) bildirimi** sağlayarak, daha şeffaf ve izlenebilir bir yapı sunar.

## 🚀 Özellikler

- **Freeze Account**: Belirli bir adresin token transfer yetkisi durdurulur.
- **Unfreeze Account**: Dondurulmuş bir adresin transfer yetkisi yeniden aktif edilir.
- **Transfer**: Donmuş hesaplardan token transferi yapılamaz.
- **Event Yayınlama**: Freeze ve Unfreeze işlemleri sırasında sistem event yayınlar (log sistemi üzerinden).
- **Minimal ve Temiz Kod Yapısı**: Fazlalıklardan arındırılmış, okunabilir ve geliştirilebilir bir altyapı.

## 🛠️ Teknik Bilgiler

- Soroban SDK kullanılarak geliştirilmiştir.
- Tüm freeze işlemleri blockchain üzerinde kalıcı olarak kaydedilir.
- Olay yönetimi sayesinde sistem hareketleri izlenebilir.
- Proje, event-driven bir mimari kullanarak daha profesyonel bir yapı sunar.

## 📦 Kullanılan Fonksiyonlar

| Fonksiyon          | Açıklama                                          |
|--------------------|----------------------------------------------------|
| `freeze_account(env, address)` | Adresin transfer yetkisini dondurur.          |
| `unfreeze_account(env, address)` | Adresin transfer yetkisini çözer.             |
| `transfer(env, from, to, amount)` | Hesaplar arası token transferi yapar.          |

## 🔗 Deployment Bilgileri

- **Network:** Stellar Testnet
- **Deploy Komutu Örneği:**
  ```bash
  soroban contract deploy --wasm target/wasm32-unknown-unknown/release/final_project_pro.wasm --network testnet --source my_wallet
  ```

- **Sözleşme Adresi:**  
  (Deploy ettikten sonra buraya `Contract ID` yazılacak)

## 👥 Geliştirici

- **Ad:** Eren Karakoyun
- **Proje Amacı:** Kursun final projesi kapsamında, orijinallik ve teknik kaliteyi artıracak bir freeze mekanizması geliştirmek.

---
