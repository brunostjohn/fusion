#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn search_test() {
        let games = search("Grand Theft Auto", false, 2).await;
        assert!(games.is_ok());
    }

    #[tokio::test]
    async fn game_test() {
        let game_page = game("grand-theft-auto-3w6").await;
        println!("{:?}", game_page);
        assert!(game_page.is_ok());
    }
}
