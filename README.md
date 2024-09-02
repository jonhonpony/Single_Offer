Kodun Amacı ve İşlevi
Bu kod, Rust programlama dilinde yazılmış bir akıllı sözleşme uygulamasıdır. Akıllı sözleşme, belirli koşullar sağlandığında otomatik olarak yürütülen, dijital olarak kodlanmış bir anlaşmadır. Bu kod, Soroban SDK kullanılarak geliştirilmiş olup, token ticareti yapmak için bir teklif (offer) oluşturma, ticaret yapma, token çekme ve teklif fiyatlarını güncelleme işlevlerini içerir.

Ana Bileşenler
DataKey Enum'u:

DataKey enum'u, akıllı sözleşmenin depolama anahtarlarını tanımlar. Bu örnekte, sadece Offer anahtarı tanımlanmıştır.
Offer Yapısı:

Offer yapısı, bir teklifin detaylarını içerir:
Seller: Teklifi oluşturan satıcının adresi.
sell_token: Satıcının satmak istediği tokenin adresi.
buy_token: Satıcının almak istediği tokenin adresi.
sell_price: Satıcının satmak istediği tokenin fiyatı.
buy_price: Satıcının almak istediği tokenin fiyatı.
SingleOffer Akıllı Sözleşmesi:

SingleOffer yapısı, akıllı sözleşmenin ana yapısını temsil eder.
create: Yeni bir teklif oluşturur. Eğer zaten bir teklif varsa veya fiyatlar sıfırsa hata fırlatır.
trade: Alıcı ve satıcı arasında token ticareti yapar. Alıcının yeterli tokeni olup olmadığını kontrol eder ve transfer işlemlerini gerçekleştirir.
withdraw: Satıcının belirli bir miktar token çekmesini sağlar.
updt_price: Teklifin fiyatlarını günceller. Fiyatlar sıfır olamaz.
get_offer: Mevcut teklifi döndürür.
Yardımcı İşlevler:

load_offer: Depolamadan mevcut teklifi yükler.
write_offer: Depolamaya yeni bir teklif yazar.
Kodun İşleyişi
Teklif Oluşturma:

create fonksiyonu, bir satıcı tarafından yeni bir teklif oluşturulmasını sağlar. Satıcı, satmak istediği tokenin adresini, almak istediği tokenin adresini ve her iki tokenin fiyatlarını belirtir. Eğer zaten bir teklif varsa veya fiyatlar sıfırsa, fonksiyon hata fırlatır.
Ticaret Yapma:

trade fonksiyonu, bir alıcı tarafından belirli bir miktar token satın alınmasını sağlar. Alıcı, almak istediği token miktarını ve minimum satmak istediği token miktarını belirtir. Fonksiyon, alıcının yeterli tokeni olup olmadığını kontrol eder ve transfer işlemlerini gerçekleştirir.
Token Çekme:

withdraw fonksiyonu, satıcının belirli bir miktar token çekmesini sağlar. Satıcı, çekmek istediği tokenin adresini ve miktarını belirtir.
Fiyat Güncelleme:

updt_price fonksiyonu, mevcut teklifin fiyatlarını günceller. Fiyatlar sıfır olamaz.
Teklif Bilgilerini Alma:

get_offer fonksiyonu, mevcut teklifin detaylarını döndürür.
Bu akıllı sözleşme, token ticareti yapmak isteyen kullanıcılar için güvenli ve otomatik bir yol sağlar. Kullanıcılar, belirli koşullar sağlandığında token ticareti yapabilir, token çekebilir ve teklif fiyatlarını güncelleyebilir.
