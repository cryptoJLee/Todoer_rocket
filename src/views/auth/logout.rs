use rocket::response::content::RawHtml;

#[get("/logout")]
pub async fn logout() -> RawHtml<&'static str> {
  RawHtml("<html>\
      <script>\
        localStorage.removeItem('user-token');\
        localStorage.removeItem('item-cache-date');\
        localStorage.removeItem('item-cache-data-pending');\
        localStorage.removeItem('item-cache-done');\
        window.location.replace(document.location.origin);\
      </script>\
      </html>")
}