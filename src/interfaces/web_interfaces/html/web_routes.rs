#[derive(Debug, Clone)]
pub struct WebRoutes {
	pub index: &'static str,
	pub results: &'static str,
	pub vote: &'static str,
}

pub const WEB_ROUTES: WebRoutes = WebRoutes {
	index: "/",
	results: "/results",
	vote: "/vote",
};
