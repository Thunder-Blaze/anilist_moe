use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    println!("Testing endpoint creation...");

    // Test all endpoints are accessible
    let _media = client.media();
    let _character = client.character();
    let _staff = client.staff();
    let _user = client.user();
    let _studio = client.studio();
    let _forum = client.forum();
    let _activity = client.activity();
    let _review = client.review();
    let _recommendation = client.recommendation();
    let _airing = client.airing();
    let _notification = client.notification();

    println!("✓ All endpoints compile successfully!");
    println!("✓ Media endpoint: Available");
    println!("✓ Character endpoint: Available");
    println!("✓ Staff endpoint: Available");
    println!("✓ User endpoint: Available");
    println!("✓ Studio endpoint: Available");
    println!("✓ Forum endpoint: Available");
    println!("✓ Activity endpoint: Available");
    println!("✓ Review endpoint: Available");
    println!("✓ Recommendation endpoint: Available");
    println!("✓ Airing endpoint: Available");
    println!("✓ Notification endpoint: Available");

    println!("\nEndpoint compilation test complete!");

    Ok(())
}
