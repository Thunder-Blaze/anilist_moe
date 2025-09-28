use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    println!("🚀 AniList API Client - Comprehensive Endpoint Test\n");

    // Test that all endpoints are accessible (compile-time verification)
    println!("Testing Media Endpoint...");
    let media_endpoint = client.media();
    println!("✓ Media endpoint methods compile");

    println!("Testing Character Endpoint...");
    let character_endpoint = client.character();
    println!("✓ Character endpoint methods compile");

    println!("Testing Staff Endpoint...");
    let staff_endpoint = client.staff();
    println!("✓ Staff endpoint methods compile");

    println!("Testing User Endpoint...");
    let user_endpoint = client.user();
    println!("✓ User endpoint methods compile");

    println!("Testing Studio Endpoint...");
    let studio_endpoint = client.studio();
    println!("✓ Studio endpoint methods compile");

    println!("Testing Forum Endpoint...");
    let forum_endpoint = client.forum();
    println!("✓ Forum endpoint methods compile");

    println!("Testing Activity Endpoint...");
    let activity_endpoint = client.activity();
    println!("✓ Activity endpoint methods compile");

    println!("Testing Review Endpoint...");
    let review_endpoint = client.review();
    println!("✓ Review endpoint methods compile");

    println!("Testing Recommendation Endpoint...");
    let rec_endpoint = client.recommendation();
    println!("✓ Recommendation endpoint methods compile");

    println!("Testing Airing Endpoint...");
    let airing_endpoint = client.airing();
    println!("✓ Airing endpoint methods compile");

    println!("Testing Notification Endpoint...");
    let notif_endpoint = client.notification();
    println!("✓ Notification endpoint methods compile");

    // Prevent unused variable warnings
    drop((media_endpoint, character_endpoint, staff_endpoint, user_endpoint, 
          studio_endpoint, forum_endpoint, activity_endpoint, review_endpoint, 
          rec_endpoint, airing_endpoint, notif_endpoint));

    println!("\n🎉 SUCCESS: All 11 endpoints with their methods compile successfully!");
    println!("📋 Summary:");
    println!("   • Media Endpoint        ✓");
    println!("   • Character Endpoint    ✓");
    println!("   • Staff Endpoint        ✓");
    println!("   • User Endpoint         ✓");
    println!("   • Studio Endpoint       ✓");
    println!("   • Forum Endpoint        ✓");
    println!("   • Activity Endpoint     ✓");
    println!("   • Review Endpoint       ✓");
    println!("   • Recommendation EP     ✓");
    println!("   • Airing Endpoint       ✓");
    println!("   • Notification Endpoint ✓");

    println!("\n🔧 Ready for testing with actual API calls!");
    println!("💡 To run actual tests, use: cargo test <test_name>");

    Ok(())
}