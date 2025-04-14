impl News
{
    pub fn get_repository(pool : sqlx :: Pool < sqlx :: Postgres >) ->
    NewsRepository { NewsRepository :: new(pool) }
} impl News
{
    pub fn base_sql() -> String { format! ("SELECT * FROM news",) } pub fn
    group_by() -> String { "".to_string() } pub fn query_builder() ->
    NewsRepositoryQueryBuilder
    {
        let base_sql = format! ("SELECT * FROM news",); let g = News ::
        group_by(); NewsRepositoryQueryBuilder :: from(& base_sql, & g)
    }
} impl NewsSummary
{
    pub fn base_sql_with(where_and_statements : & str) -> String
    {
        tracing :: debug!
        ("{} base_sql_with group: {}",
        "SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, category, title, image, contents, main FROM news",
        ""); let query = if where_and_statements.is_empty()
        {
            format!
            ("{} {}", format!
            ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, category, title, image, contents, main FROM news",),
            "")
        } else
        {
            if where_and_statements.to_lowercase().starts_with("where")
            {
                format!
                ("{} {} {}", format!
                ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, category, title, image, contents, main FROM news",),
                where_and_statements, "")
            } else
            {
                format!
                ("{} WHERE {} {}", format!
                ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, category, title, image, contents, main FROM news",),
                where_and_statements, "")
            }
        }; query
    } pub fn query_builder() -> NewsRepositoryQueryBuilder
    {
        let base_sql = format!
        ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, category, title, image, contents, main FROM news",);
        NewsRepositoryQueryBuilder :: from(& base_sql, "").with_count()
    }
} #[derive(Debug, Clone)] pub struct NewsRepository
{ pool : sqlx :: Pool < sqlx :: Postgres > , }
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
NewsRepositoryUpdateRequest
{
    pub category : Option < String > , pub title : Option < String > , pub
    image : Option < String > , pub contents : Option < String > , pub main :
    Option < bool >
} impl NewsRepositoryUpdateRequest
{
    pub fn new() -> Self { Self :: default() } pub fn
    with_category(mut self, category : String) -> Self
    { self.category = Some(category); self } pub fn
    with_title(mut self, title : String) -> Self
    { self.title = Some(title); self } pub fn
    with_image(mut self, image : String) -> Self
    { self.image = Some(image); self } pub fn
    with_contents(mut self, contents : String) -> Self
    { self.contents = Some(contents); self } pub fn
    with_main(mut self, main : bool) -> Self { self.main = Some(main); self }
} impl NewsRepository
{
    pub fn new(pool : sqlx :: Pool < sqlx :: Postgres >) -> Self
    { Self { pool } } pub fn queries(& self) -> Vec < & 'static str >
    {
        vec!
        ["CREATE TABLE IF NOT EXISTS news (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,category TEXT NOT NULL,title TEXT NOT NULL,image TEXT NOT NULL,contents TEXT NOT NULL,main BOOLEAN NOT NULL);",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_news'\n        AND tgrelid = 'news'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_news\n        BEFORE INSERT ON news\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_news'\n        AND tgrelid = 'news'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_news\n        BEFORE INSERT OR UPDATE ON news\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$"]
    } pub async fn create_this_table(& self) -> std :: result :: Result < (),
    sqlx :: Error >
    {
        tracing :: trace!
        ("Create table: {}",
        "CREATE TABLE IF NOT EXISTS news (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,category TEXT NOT NULL,title TEXT NOT NULL,image TEXT NOT NULL,contents TEXT NOT NULL,main BOOLEAN NOT NULL);");
        sqlx ::
        query("CREATE TABLE IF NOT EXISTS news (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,category TEXT NOT NULL,title TEXT NOT NULL,image TEXT NOT NULL,contents TEXT NOT NULL,main BOOLEAN NOT NULL);").execute(&
        self.pool).await ? ; Ok(())
    } pub async fn create_related_tables(& self) -> std :: result :: Result <
    (), sqlx :: Error >
    {
        for query in
        ["DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_news'\n        AND tgrelid = 'news'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_news\n        BEFORE INSERT ON news\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_news'\n        AND tgrelid = 'news'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_news\n        BEFORE INSERT OR UPDATE ON news\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$"]
        {
            tracing :: trace! ("Execute queries: {}", query); sqlx ::
            query(query).execute(& self.pool).await ? ;
        } Ok(())
    } pub async fn create_table(& self) -> std :: result :: Result < (), sqlx
    :: Error >
    {
        sqlx ::
        query("CREATE TABLE IF NOT EXISTS news (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,category TEXT NOT NULL,title TEXT NOT NULL,image TEXT NOT NULL,contents TEXT NOT NULL,main BOOLEAN NOT NULL);").execute(&
        self.pool).await ? ; for query in
        ["DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_news'\n        AND tgrelid = 'news'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_news\n        BEFORE INSERT ON news\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_news'\n        AND tgrelid = 'news'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_news\n        BEFORE INSERT OR UPDATE ON news\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$"]
        {
            tracing :: trace! ("Execute queries: {}", query); sqlx ::
            query(query).execute(& self.pool).await ? ;
        } Ok(())
    } pub async fn drop_table(& self) -> std :: result :: Result < (), sqlx ::
    Error >
    {
        sqlx :: query("DROP TABLE IF EXISTS news;").execute(& self.pool).await
        ? ; Ok(())
    } pub async fn
    insert(& self, category : String, title : String, image : String, contents
    : String, main : bool) -> crate::Result < News >
    {
        tracing :: trace!
        ("insert query: {}",
        "INSERT INTO news (category, title, image, contents, main) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, category, title, image, contents, main");
        let row = sqlx ::
        query("INSERT INTO news (category, title, image, contents, main) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, category, title, image, contents, main").bind(category).bind(title).bind(image).bind(contents).bind(main).map(|
        row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; News
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), category :
                row.try_get("category").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), contents :
                row.try_get("contents").unwrap_or_default(), main :
                row.try_get("main").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn insert_with_tx < 'e, 'c : 'e, E >
    (& self, tx : E, category : String, title : String, image : String,
    contents : String, main : bool) -> crate::Result < Option < News >> where
    E : sqlx :: Executor < 'c, Database = sqlx :: postgres :: Postgres > ,
    {
        tracing :: trace!
        ("insert query: {}",
        "INSERT INTO news (category, title, image, contents, main) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, category, title, image, contents, main");
        let row = sqlx ::
        query("INSERT INTO news (category, title, image, contents, main) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, category, title, image, contents, main").bind(category).bind(title).bind(image).bind(contents).bind(main).map(|
        row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; News
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), category :
                row.try_get("category").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), contents :
                row.try_get("contents").unwrap_or_default(), main :
                row.try_get("main").unwrap_or_default()
            }
        }).fetch_optional(tx).await ? ; Ok(row)
    } pub async fn
    insert_without_returning(& self, category : String, title : String, image
    : String, contents : String, main : bool) -> crate::Result < () >
    {
        tracing :: trace!
        ("insert query: {}",
        "INSERT INTO news (category, title, image, contents, main) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, category, title, image, contents, main");
        sqlx ::
        query("INSERT INTO news (category, title, image, contents, main) VALUES ($1, $2, $3, $4, $5)").bind(category).bind(title).bind(image).bind(contents).bind(main).execute(&
        self.pool).await ? ; Ok(())
    } pub async fn
    update(& self, id : i64, news_repository_update_request :
    NewsRepositoryUpdateRequest) -> crate::Result < News >
    {
        let mut i = 1; let mut update_values = vec! []; if
        news_repository_update_request.category.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "category", i)); }
        if news_repository_update_request.title.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "title", i)); } if
        news_repository_update_request.image.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "image", i)); } if
        news_repository_update_request.contents.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "contents", i)); }
        if news_repository_update_request.main.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "main", i)); } let
        query = format!
        ("UPDATE news SET {} WHERE id = $1 RETURNING id, created_at, updated_at, category, title, image, contents, main",
        update_values.join(", "),); tracing :: trace!
        ("insert query: {}", query); let mut q = sqlx ::
        query(& query).bind(id); if let Some(category) =
        news_repository_update_request.category { q = q.bind(category); } if
        let Some(title) = news_repository_update_request.title
        { q = q.bind(title); } if let Some(image) =
        news_repository_update_request.image { q = q.bind(image); } if let
        Some(contents) = news_repository_update_request.contents
        { q = q.bind(contents); } if let Some(main) =
        news_repository_update_request.main { q = q.bind(main); } let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; News
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), category :
                row.try_get("category").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), contents :
                row.try_get("contents").unwrap_or_default(), main :
                row.try_get("main").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn update_with_tx < 'e, 'c : 'e, E >
    (& self, tx : E, id : i64, news_repository_update_request :
    NewsRepositoryUpdateRequest) -> crate::Result < Option < News >> where E :
    sqlx :: Executor < 'c, Database = sqlx :: postgres :: Postgres > ,
    {
        let mut i = 1; let mut update_values = vec! []; if
        news_repository_update_request.category.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "category", i)); }
        if news_repository_update_request.title.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "title", i)); } if
        news_repository_update_request.image.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "image", i)); } if
        news_repository_update_request.contents.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "contents", i)); }
        if news_repository_update_request.main.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "main", i)); } let
        query = format!
        ("UPDATE news SET {} WHERE id = $1 RETURNING id, created_at, updated_at, category, title, image, contents, main",
        update_values.join(", "),); tracing :: trace!
        ("insert query: {}", query); let mut q = sqlx ::
        query(& query).bind(id); if let Some(category) =
        news_repository_update_request.category { q = q.bind(category); } if
        let Some(title) = news_repository_update_request.title
        { q = q.bind(title); } if let Some(image) =
        news_repository_update_request.image { q = q.bind(image); } if let
        Some(contents) = news_repository_update_request.contents
        { q = q.bind(contents); } if let Some(main) =
        news_repository_update_request.main { q = q.bind(main); } let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; News
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), category :
                row.try_get("category").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), contents :
                row.try_get("contents").unwrap_or_default(), main :
                row.try_get("main").unwrap_or_default()
            }
        }).fetch_optional(tx).await ? ; Ok(row)
    } pub async fn delete(& self, id : i64) -> crate::Result < News >
    {
        let res = sqlx ::
        query("DELETE FROM news WHERE id = $1 RETURNING *").bind(id).map(News
        :: from).fetch_one(& self.pool).await ? ; Ok(res)
    } pub async fn delete_with_tx < 'e, 'c : 'e, E >
    (& self, tx : E, id : i64) -> crate::Result < Option < News >> where E :
    sqlx :: Executor < 'c, Database = sqlx :: postgres :: Postgres > ,
    {
        let res = sqlx ::
        query("DELETE FROM news WHERE id = $1 RETURNING *").bind(id).map(News
        :: from).fetch_optional(tx).await ? ; Ok(res)
    } pub async fn find_one(& self, param : & NewsReadAction) -> crate::Result
    < News >
    {
        let mut query = format! ("{}", News :: base_sql());
        query.push_str(" "); query.push_str(News :: group_by().as_str());
        tracing :: trace!
        ("{} query {}: {:?}", "NewsRepository::find_one", query, param); let
        mut q = sqlx :: query(& query); let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; News
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), category :
                row.try_get("category").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), contents :
                row.try_get("contents").unwrap_or_default(), main :
                row.try_get("main").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn find(& self, param : & NewsQuery) -> crate::Result <
    by_types::QueryResponse<NewsSummary> >
    {
        let query = format!
        ("WITH data AS ({} {}) SELECT ({}) AS total_count, data.* FROM data;",
        "SELECT * FROM news", "LIMIT $1 OFFSET $2",
        "SELECT COUNT(*) FROM news"); tracing :: trace!
        ("{} query {}", "NewsRepository::find_one", query); let offset : i32 =
        (param.size as i32) * (param.page() - 1); let q = sqlx ::
        query(& query).bind(param.size as i32).bind(offset); let mut total :
        i64 = 0; let rows =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; total = row.get("total_count"); row.into()
        }).fetch_all(& self.pool).await ? ; Ok((rows, total).into())
    }
} impl From < sqlx :: postgres :: PgRow > for News
{
    fn from(row : sqlx :: postgres :: PgRow) -> Self
    {
        use sqlx :: Row; News
        {
            id : row.try_get("id").unwrap_or_default(), created_at :
            row.try_get("created_at").unwrap_or_default(), updated_at :
            row.try_get("updated_at").unwrap_or_default(), category :
            row.try_get("category").unwrap_or_default(), title :
            row.try_get("title").unwrap_or_default(), image :
            row.try_get("image").unwrap_or_default(), contents :
            row.try_get("contents").unwrap_or_default(), main :
            row.try_get("main").unwrap_or_default()
        }
    }
} impl From < sqlx :: postgres :: PgRow > for NewsSummary
{
    fn from(row : sqlx :: postgres :: PgRow) -> Self
    {
        use sqlx :: Row; NewsSummary
        {
            id : row.try_get("id").unwrap_or_default(), created_at :
            row.try_get("created_at").unwrap_or_default(), updated_at :
            row.try_get("updated_at").unwrap_or_default(), category :
            row.try_get("category").unwrap_or_default(), title :
            row.try_get("title").unwrap_or_default(), image :
            row.try_get("image").unwrap_or_default(), contents :
            row.try_get("contents").unwrap_or_default(), main :
            row.try_get("main").unwrap_or_default()
        }
    }
} #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default)]
pub struct NewsRepositoryQueryBuilder
{
    pub base_sql : String, pub group_by : String, pub count : bool, pub
    conditions : Vec < by_types :: Conditions > , pub order : by_types ::
    Order, pub limit : Option < i32 > , pub page : Option < i32 > , pub or :
    Vec < Vec < by_types :: Conditions >> ,
} impl std :: ops :: BitOr for NewsRepositoryQueryBuilder
{
    type Output = Self; fn bitor(self, rhs : Self) -> Self :: Output
    {
        let mut new_or = self.or; new_or.push(rhs.conditions); if !
        rhs.or.is_empty() { new_or.extend(rhs.or); } Self
        {
            base_sql : self.base_sql, group_by : self.group_by, count :
            self.count || rhs.count, conditions : self.conditions, order :
            self.order, limit : self.limit.or(rhs.limit), page :
            self.page.or(rhs.page), or : new_or,
        }
    }
} impl std :: ops :: BitOrAssign for NewsRepositoryQueryBuilder
{
    fn bitor_assign(& mut self, rhs : Self)
    {
        self.or.push(rhs.conditions); if ! rhs.or.is_empty()
        { self.or.extend(rhs.or); } self.count = self.count || rhs.count;
        self.limit = self.limit.or(rhs.limit); self.page =
        self.page.or(rhs.page);
    }
} impl NewsRepositoryQueryBuilder
{
    pub fn from(base_sql : & str, group_by : & str) -> Self
    {
        Self
        {
            base_sql : base_sql.to_string(), group_by : group_by.to_string(),
            .. Default :: default()
        }
    } pub fn with_count(mut self) -> Self { self.count = true; self } pub fn
    new() -> Self { Self :: default() } pub fn limit(mut self, limit : i32) ->
    Self { self.limit = Some(limit); self } pub fn page(mut self, page : i32)
    -> Self { self.page = Some(page); self } pub fn
    build_where_starts_with(& self, i : & mut i32) -> String
    {
        let mut where_clause = vec! []; tracing :: debug!
        ("Building where clause for {}", "NewsRepositoryQueryBuilder"); let
        prefix = if self.group_by.is_empty() { "" } else { "p." }; for
        condition in self.conditions.iter()
        {
            let (q, new_i) = condition.to_binder(* i); * i = new_i;
            where_clause.push(format! ("{}{}", prefix, q));
        } tracing :: debug! ("conditions: {:?}", where_clause); let mut ret =
        vec! [where_clause.join(" AND ")]; for conditions in self.or.iter()
        {
            let mut where_clause = vec! []; for condition in conditions.iter()
            {
                let (q, new_i) = condition.to_binder(* i); * i = new_i;
                where_clause.push(format! ("{}{}", prefix, q));
            } ret.push(where_clause.join(" AND "));
        } if ret.len() == 1 { ret [0].clone() } else
        { format! ("({})", ret.join(") OR (")) }
    } pub fn build_where(& self) -> String
    { let mut _i = 1; self.build_where_starts_with(& mut _i) } pub fn
    sql_starts_with(& self, i : & mut i32) -> String
    {
        let w = self.build_where_starts_with(i); let mut query = if
        w.is_empty()
        { format! ("{} {} {}", self.base_sql, self.group_by, self.order) }
        else
        {
            format!
            ("{} WHERE {} {} {}", self.base_sql, w, self.group_by, self.order)
        }; if self.count && !
        query.starts_with("SELECT COUNT(*) OVER() as total_count")
        {
            query =
            query.replacen("SELECT", "SELECT COUNT(*) OVER() as total_count,",
            1);
        } let ret = if let Some(limit) = self.limit
        {
            if let Some(page) = self.page
            {
                format!
                ("{} LIMIT {} OFFSET {}", query, limit, (limit * (page - 1)))
            } else { format! ("{} LIMIT {}", query, limit) }
        } else { query }; ret
    } pub fn sql(& self) -> String
    { let mut _i = 1; self.sql_starts_with(& mut _i) } pub fn
    all_conditions(& self) -> Vec < by_types :: Conditions >
    {
        let mut conditions = self.conditions.clone();
        conditions.extend(self.or.iter().flatten().cloned()); conditions
    } pub fn query(& self,) -> sqlx :: query :: Query < 'static, sqlx ::
    Postgres, < sqlx :: Postgres as sqlx :: Database > :: Arguments < 'static
    > , >
    {
        let mut i = 1; let mut query = self.sql_starts_with(& mut i); let s :
        Box < String > = Box :: new(query); let query : & 'static str = Box ::
        leak(s); let mut q = sqlx :: query(query); for condition in
        self.all_conditions()
        {
            q = match condition
            {
                by_types :: Conditions :: BetweenBigint(_, from, to) =>
                {
                    tracing :: debug!
                    ("Binding BetweenBigint {} and {}", from, to);
                    q.bind(from).bind(to)
                }, by_types :: Conditions :: EqualsBigint(_, value) =>
                {
                    tracing :: debug! ("Binding EqualsBigint {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: NotEqualsBigint(_, value) =>
                {
                    tracing :: debug! ("Binding NotEqualsBigint {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: GreaterThanBigint(_, value) =>
                {
                    tracing :: debug! ("Binding GreaterThanBigint {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: LessThanBigint(_, value) =>
                {
                    tracing :: debug! ("Binding LessThanBigint {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: GreaterThanEqualsBigint(_, value)
                =>
                {
                    tracing :: debug!
                    ("Binding GreaterThanEqualsBigint {}", value); q.bind(value)
                }, by_types :: Conditions :: LessThanEqualsBigint(_, value) =>
                {
                    tracing :: debug!
                    ("Binding LessThanEqualsBigint {}", value); q.bind(value)
                }, by_types :: Conditions :: BetweenInteger(_, from, to) =>
                {
                    tracing :: debug!
                    ("Binding BetweenInteger {} and {}", from, to);
                    q.bind(from).bind(to)
                }, by_types :: Conditions :: EqualsInteger(_, value) =>
                {
                    tracing :: debug! ("Binding EqualsInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: NotEqualsInteger(_, value) =>
                {
                    tracing :: debug! ("Binding NotEqualsInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: GreaterThanInteger(_, value) =>
                {
                    tracing :: debug! ("Binding GreaterThanInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: LessThanInteger(_, value) =>
                {
                    tracing :: debug! ("Binding LessThanInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions ::
                GreaterThanEqualsInteger(_, value) =>
                {
                    tracing :: debug!
                    ("Binding GreaterThanEqualsInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: LessThanEqualsInteger(_, value)
                =>
                {
                    tracing :: debug!
                    ("Binding LessThanEqualsInteger {}", value); q.bind(value)
                }, by_types :: Conditions :: EqualsText(_, value) =>
                {
                    tracing :: debug! ("Binding EqualsText {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: NotEqualsText(_, value) =>
                {
                    tracing :: debug! ("Binding NotEqualsText {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: ContainsText(_, value) =>
                {
                    let value = format! ("%{}%", value); tracing :: debug!
                    ("Binding ContainsText {}", value); q.bind(value)
                }, by_types :: Conditions :: NotContainsText(_, value) =>
                {
                    let value = format! ("%{}%", value); tracing :: debug!
                    ("Binding NotContainsText {}", value); q.bind(value)
                } by_types :: Conditions :: StartsWithText(_, value) =>
                {
                    let value = format! ("{}%", value); tracing :: debug!
                    ("Binding StartsWithText {}", value); q.bind(value)
                } by_types :: Conditions :: NotStartsWithText(_, value) =>
                {
                    let value = format! ("{}%", value); tracing :: debug!
                    ("Binding NotStartsWithText {}", value); q.bind(value)
                } by_types :: Conditions :: EndsWithText(_, value) =>
                {
                    let value = format! ("%{}", value); tracing :: debug!
                    ("Binding EndsWithText {}", value); q.bind(value)
                } by_types :: Conditions :: NotEndsWithText(_, value) =>
                {
                    let value = format! ("%{}", value); tracing :: debug!
                    ("Binding NotEndsWithText {}", value); q.bind(value)
                } by_types :: Conditions :: TrueBoolean(_) =>
                { tracing :: debug! ("(Not)Binding TrueBoolean"); q } by_types
                :: Conditions :: FalseBoolean(_) =>
                { tracing :: debug! ("(Not)Binding FalseBoolean"); q }
                by_types :: Conditions :: Custom(_) =>
                { tracing :: debug! ("(Not)Binding FalseBoolean"); q }
            };
        } q
    } pub fn id_equals(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsBigint("id".to_string(), id)); self
    } pub fn id_not_equals(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsBigint("id".to_string(), id)); self
    } pub fn id_greater_than(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanBigint("id".to_string(), id)); self
    } pub fn id_less_than(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanBigint("id".to_string(), id)); self
    } pub fn id_greater_than_equals(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanEqualsBigint("id".to_string(), id)); self
    } pub fn id_less_than_equals(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanEqualsBigint("id".to_string(), id)); self
    } pub fn id_between(mut self, from : i64, to : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        BetweenBigint("id".to_string(), from, to)); self
    } pub fn order_by_id_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "id")); } else
        { self.order = by_types :: Order :: Asc(vec! ["id".to_string()]); }
        self
    } pub fn order_by_id_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "id")); } else
        { self.order = by_types :: Order :: Desc(vec! ["id".to_string()]); }
        self
    } pub fn created_at_equals(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_not_equals(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_greater_than(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_less_than(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_greater_than_equals(mut self, created_at : i64) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanEqualsBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_less_than_equals(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanEqualsBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_between(mut self, from : i64, to : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        BetweenBigint("created_at".to_string(), from, to)); self
    } pub fn order_by_created_at_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "created_at")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["created_at".to_string()]);
        } self
    } pub fn order_by_created_at_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "created_at")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["created_at".to_string()]);
        } self
    } pub fn updated_at_equals(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_not_equals(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_greater_than(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_less_than(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_greater_than_equals(mut self, updated_at : i64) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanEqualsBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_less_than_equals(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanEqualsBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_between(mut self, from : i64, to : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        BetweenBigint("updated_at".to_string(), from, to)); self
    } pub fn order_by_updated_at_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "updated_at")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["updated_at".to_string()]);
        } self
    } pub fn order_by_updated_at_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "updated_at")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["updated_at".to_string()]);
        } self
    } pub fn category_equals(mut self, category : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("category".to_string(), category)); self
    } pub fn category_not_equals(mut self, category : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("category".to_string(), category)); self
    } pub fn category_contains(mut self, category : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("category".to_string(), category)); self
    } pub fn category_not_contains(mut self, category : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("category".to_string(), category)); self
    } pub fn category_starts_with(mut self, category : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("category".to_string(), category)); self
    } pub fn category_not_starts_with(mut self, category : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("category".to_string(), category)); self
    } pub fn category_ends_with(mut self, category : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("category".to_string(), category)); self
    } pub fn category_not_ends_with(mut self, category : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("category".to_string(), category)); self
    } pub fn order_by_category_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "category")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["category".to_string()]);
        } self
    } pub fn order_by_category_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "category")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["category".to_string()]);
        } self
    } pub fn title_equals(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("title".to_string(), title)); self
    } pub fn title_not_equals(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("title".to_string(), title)); self
    } pub fn title_contains(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("title".to_string(), title)); self
    } pub fn title_not_contains(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("title".to_string(), title)); self
    } pub fn title_starts_with(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("title".to_string(), title)); self
    } pub fn title_not_starts_with(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("title".to_string(), title)); self
    } pub fn title_ends_with(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("title".to_string(), title)); self
    } pub fn title_not_ends_with(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("title".to_string(), title)); self
    } pub fn order_by_title_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "title")); } else
        { self.order = by_types :: Order :: Asc(vec! ["title".to_string()]); }
        self
    } pub fn order_by_title_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "title")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["title".to_string()]);
        } self
    } pub fn image_equals(mut self, image : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("image".to_string(), image)); self
    } pub fn image_not_equals(mut self, image : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("image".to_string(), image)); self
    } pub fn image_contains(mut self, image : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("image".to_string(), image)); self
    } pub fn image_not_contains(mut self, image : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("image".to_string(), image)); self
    } pub fn image_starts_with(mut self, image : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("image".to_string(), image)); self
    } pub fn image_not_starts_with(mut self, image : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("image".to_string(), image)); self
    } pub fn image_ends_with(mut self, image : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("image".to_string(), image)); self
    } pub fn image_not_ends_with(mut self, image : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("image".to_string(), image)); self
    } pub fn order_by_image_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "image")); } else
        { self.order = by_types :: Order :: Asc(vec! ["image".to_string()]); }
        self
    } pub fn order_by_image_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "image")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["image".to_string()]);
        } self
    } pub fn contents_equals(mut self, contents : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("contents".to_string(), contents)); self
    } pub fn contents_not_equals(mut self, contents : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("contents".to_string(), contents)); self
    } pub fn contents_contains(mut self, contents : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("contents".to_string(), contents)); self
    } pub fn contents_not_contains(mut self, contents : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("contents".to_string(), contents)); self
    } pub fn contents_starts_with(mut self, contents : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("contents".to_string(), contents)); self
    } pub fn contents_not_starts_with(mut self, contents : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("contents".to_string(), contents)); self
    } pub fn contents_ends_with(mut self, contents : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("contents".to_string(), contents)); self
    } pub fn contents_not_ends_with(mut self, contents : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("contents".to_string(), contents)); self
    } pub fn order_by_contents_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "contents")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["contents".to_string()]);
        } self
    } pub fn order_by_contents_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "contents")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["contents".to_string()]);
        } self
    } pub fn main_is_true(mut self) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        TrueBoolean("main".to_string())); self
    } pub fn main_is_false(mut self) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        FalseBoolean("main".to_string())); self
    } pub fn order_by_random(mut self) -> Self
    { self.order = by_types :: Order :: Random; self }
} /// News is a generated struct that represents the model
///
/// For making API calls related to this model, use `News::get_client(endpoint: &str)`.
/// It will returns NewsClient struct that implements the API calls.
///
/// In server side, you can use `News::get_repository()` to interact with the database.
/// Recommend to use `NewsRepository` to insert or update the model.
/// To query the model, use `News::query_builder()`.
/// For more detail, refer to the documentation of the query builder.
#[derive(Debug, Clone, serde :: Deserialize, serde :: Serialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct News
{
    pub id : i64, pub created_at : i64, pub updated_at : i64, pub category :
    String, pub title : String, pub image : String,
    #[validate(length(max = 350))] pub contents : String, pub main : bool
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo, sqlx :: FromRow))] pub
struct NewsSummary
{
    pub id : i64, pub created_at : i64, pub updated_at : i64, pub category :
    String, pub title : String, pub image : String, pub contents : String, pub
    main : bool,
} impl From < News > for NewsSummary
{
    fn from(item : News) -> Self
    {
        Self
        {
            id : item.id, created_at : item.created_at, updated_at :
            item.updated_at, category : item.category, title : item.title,
            image : item.image, contents : item.contents, main : item.main,
        }
    }
} impl Into < News > for NewsSummary
{
    fn into(self) -> News
    {
        News
        {
            id : self.id, created_at : self.created_at, updated_at :
            self.updated_at, category : self.category, title : self.title,
            image : self.image, contents : self.contents, main : self.main, ..
            Default :: default()
        }
    }
} #[derive(validator :: Validate)]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq, by_macros :: QueryDisplay)] #[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct NewsQuery
{
    #[serde(deserialize_with = "parse_size_of_news_query", default)] pub size
    : usize, pub bookmark : Option < String > ,
} pub fn parse_size_of_news_query < 'de, D > (deserializer : D) -> std ::
result :: Result < usize, D :: Error > where D : serde :: Deserializer < 'de >
,
{
    use serde :: Deserialize; let s : Option < String > = Option ::
    deserialize(deserializer) ? ;
    s.unwrap_or_else(|| Default :: default()).parse :: < usize >
    ().map_err(serde :: de :: Error :: custom)
} impl NewsQuery
{
    pub fn new(size : usize) -> Self { Self { size, .. Self :: default() } }
    pub fn with_bookmark(mut self, bookmark : String) -> Self
    { self.bookmark = Some(bookmark); self } pub fn
    with_page(mut self, page : usize) -> Self
    { self.bookmark = Some(page.to_string()); self }
    #[doc = r" Returns the size(i32) of the query"] pub fn size(& self) -> i32
    { self.size as i32 } pub fn page(& self) -> i32
    {
        self.bookmark.as_ref().unwrap_or(&
        "1".to_string()).parse().unwrap_or(1)
    }
} impl NewsClient {} impl News
{
    pub fn get_client(endpoint : & str) -> NewsClient
    { NewsClient { endpoint : endpoint.to_string() } }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)] pub struct NewsClient { pub endpoint : String, } impl NewsClient
{
    pub async fn query(& self, params : NewsQuery,) -> crate::Result <
    by_types::QueryResponse<NewsSummary> >
    {
        let path = format! ("/v1/news",); let endpoint = format!
        ("{}{}", self.endpoint, path); let query = format!
        ("{}?{}", endpoint, NewsParam :: Query(params)); rest_api ::
        get(& query).await
    } pub async fn get(& self, id : i64) -> crate::Result < News >
    {
        let path = format! ("/v1/news",); let endpoint = format!
        ("{}{}/{}", self.endpoint, path, id); rest_api ::
        get(& endpoint).await
    }
} impl News { pub fn url() -> String { "/v1/news".to_string() } }
#[derive(validator :: Validate)]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq, by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
NewsReadAction {} impl NewsReadAction
{ pub fn new() -> Self { Self :: default() } } impl NewsClient {}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, PartialEq,
by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server", derive(aide :: OperationIo))]
#[serde(tag = "param-type", rename_all = "kebab-case")] pub enum NewsParam
{ Query(NewsQuery), } #[cfg(feature = "server")] impl schemars :: JsonSchema
for NewsParam
{
    fn schema_name() -> String { "NewsParam".to_string() } fn
    json_schema(_gen : & mut schemars :: gen :: SchemaGenerator) -> schemars
    :: schema :: Schema
    {
        let mut schema_obj = schemars :: schema :: SchemaObject :: default();
        schema_obj.metadata =
        Some(Box ::
        new(schemars :: schema :: Metadata
        {
            title : Some("News Query Parameters".to_string()), .. Default ::
            default()
        }));
        schema_obj.object().properties.insert("size".to_string(), schemars ::
        schema :: Schema ::
        Object(schemars :: schema :: SchemaObject
        {
            metadata :
            Some(Box ::
            new(schemars :: schema :: Metadata
            {
                description : Some("Number of items to return".to_string()),
                .. Default :: default()
            })), instance_type :
            Some(schemars :: schema :: InstanceType :: Integer.into()), ..
            Default :: default()
        }),);
        schema_obj.object().properties.insert("bookmark".to_string(), schemars
        :: schema :: Schema ::
        Object(schemars :: schema :: SchemaObject
        {
            metadata :
            Some(Box ::
            new(schemars :: schema :: Metadata
            {
                description :
                Some("bookmark of page number. Note that you must stringify page number.".to_string()),
                default :
                Some(serde_json :: Value :: String("1".to_string())), ..
                Default :: default()
            })), instance_type :
            Some(schemars :: schema :: InstanceType :: String.into()), ..
            Default :: default()
        }),); schema_obj.object().required.insert("size".to_string());
        schemars :: schema :: Schema :: Object(schema_obj)
    }
} #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(tag = "param_type")] #[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum NewsGetResponse
{ Query(by_types::QueryResponse<NewsSummary>), }