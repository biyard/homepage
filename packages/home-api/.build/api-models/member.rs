impl Member
{
    pub fn get_repository(pool : sqlx :: Pool < sqlx :: Postgres >) ->
    MemberRepository { MemberRepository :: new(pool) }
} impl Member
{
    pub fn base_sql() -> String { format! ("SELECT * FROM members",) } pub fn
    group_by() -> String { "".to_string() } pub fn query_builder() ->
    MemberRepositoryQueryBuilder
    {
        let base_sql = format! ("SELECT * FROM members",); let g = Member ::
        group_by(); MemberRepositoryQueryBuilder :: from(& base_sql, & g)
    }
} impl MemberSummary
{
    pub fn base_sql_with(where_and_statements : & str) -> String
    {
        tracing :: debug!
        ("{} base_sql_with group: {}",
        "SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name, image, role, email, web, linkedin, github, description FROM members",
        ""); let query = if where_and_statements.is_empty()
        {
            format!
            ("{} {}", format!
            ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name, image, role, email, web, linkedin, github, description FROM members",),
            "")
        } else
        {
            if where_and_statements.to_lowercase().starts_with("where")
            {
                format!
                ("{} {} {}", format!
                ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name, image, role, email, web, linkedin, github, description FROM members",),
                where_and_statements, "")
            } else
            {
                format!
                ("{} WHERE {} {}", format!
                ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name, image, role, email, web, linkedin, github, description FROM members",),
                where_and_statements, "")
            }
        }; query
    } pub fn query_builder() -> MemberRepositoryQueryBuilder
    {
        let base_sql = format!
        ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name, image, role, email, web, linkedin, github, description FROM members",);
        MemberRepositoryQueryBuilder :: from(& base_sql, "").with_count()
    }
} #[derive(Debug, Clone)] pub struct MemberRepository
{ pool : sqlx :: Pool < sqlx :: Postgres > , }
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
MemberRepositoryUpdateRequest
{
    pub name : Option < String > , pub image : Option < String > , pub role :
    Option < MemberRole > , pub email : Option < String > , pub web : Option <
    String > , pub linkedin : Option < String > , pub github : Option < String
    > , pub description : Option < String >
} impl MemberRepositoryUpdateRequest
{
    pub fn new() -> Self { Self :: default() } pub fn
    with_name(mut self, name : String) -> Self
    { self.name = Some(name); self } pub fn
    with_image(mut self, image : String) -> Self
    { self.image = Some(image); self } pub fn
    with_role(mut self, role : MemberRole) -> Self
    { self.role = Some(role); self } pub fn
    with_email(mut self, email : String) -> Self
    { self.email = Some(email); self } pub fn with_web(mut self, web : String)
    -> Self { self.web = Some(web); self } pub fn
    with_linkedin(mut self, linkedin : String) -> Self
    { self.linkedin = Some(linkedin); self } pub fn
    with_github(mut self, github : String) -> Self
    { self.github = Some(github); self } pub fn
    with_description(mut self, description : String) -> Self
    { self.description = Some(description); self }
} impl MemberRepository
{
    pub fn new(pool : sqlx :: Pool < sqlx :: Postgres >) -> Self
    { Self { pool } } pub fn queries(& self) -> Vec < & 'static str >
    {
        vec!
        ["CREATE TABLE IF NOT EXISTS members (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,name TEXT NOT NULL,image TEXT NOT NULL,role INTEGER NOT NULL,email TEXT NOT NULL UNIQUE,web TEXT NULL,linkedin TEXT NULL,github TEXT NULL,description TEXT NOT NULL);",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_members'\n        AND tgrelid = 'members'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_members\n        BEFORE INSERT ON members\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_members'\n        AND tgrelid = 'members'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_members\n        BEFORE INSERT OR UPDATE ON members\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$"]
    } pub async fn create_this_table(& self) -> std :: result :: Result < (),
    sqlx :: Error >
    {
        tracing :: trace!
        ("Create table: {}",
        "CREATE TABLE IF NOT EXISTS members (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,name TEXT NOT NULL,image TEXT NOT NULL,role INTEGER NOT NULL,email TEXT NOT NULL UNIQUE,web TEXT NULL,linkedin TEXT NULL,github TEXT NULL,description TEXT NOT NULL);");
        sqlx ::
        query("CREATE TABLE IF NOT EXISTS members (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,name TEXT NOT NULL,image TEXT NOT NULL,role INTEGER NOT NULL,email TEXT NOT NULL UNIQUE,web TEXT NULL,linkedin TEXT NULL,github TEXT NULL,description TEXT NOT NULL);").execute(&
        self.pool).await ? ; Ok(())
    } pub async fn create_related_tables(& self) -> std :: result :: Result <
    (), sqlx :: Error >
    {
        for query in
        ["DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_members'\n        AND tgrelid = 'members'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_members\n        BEFORE INSERT ON members\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_members'\n        AND tgrelid = 'members'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_members\n        BEFORE INSERT OR UPDATE ON members\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$"]
        {
            tracing :: trace! ("Execute queries: {}", query); sqlx ::
            query(query).execute(& self.pool).await ? ;
        } Ok(())
    } pub async fn create_table(& self) -> std :: result :: Result < (), sqlx
    :: Error >
    {
        sqlx ::
        query("CREATE TABLE IF NOT EXISTS members (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,name TEXT NOT NULL,image TEXT NOT NULL,role INTEGER NOT NULL,email TEXT NOT NULL UNIQUE,web TEXT NULL,linkedin TEXT NULL,github TEXT NULL,description TEXT NOT NULL);").execute(&
        self.pool).await ? ; for query in
        ["DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_members'\n        AND tgrelid = 'members'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_members\n        BEFORE INSERT ON members\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_members'\n        AND tgrelid = 'members'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_members\n        BEFORE INSERT OR UPDATE ON members\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$"]
        {
            tracing :: trace! ("Execute queries: {}", query); sqlx ::
            query(query).execute(& self.pool).await ? ;
        } Ok(())
    } pub async fn drop_table(& self) -> std :: result :: Result < (), sqlx ::
    Error >
    {
        sqlx ::
        query("DROP TABLE IF EXISTS members;").execute(& self.pool).await ? ;
        Ok(())
    } pub async fn
    insert(& self, name : String, image : String, role : MemberRole, email :
    String, web : Option < String > , linkedin : Option < String > , github :
    Option < String > , description : String) -> crate::Result < Member >
    {
        let mut i = 5; let mut insert_fields = vec!
        ["name", "image", "role", "email", "description"]; let mut
        insert_values = vec!
        ["$1", "$2", "$3", "$4", "$5"].iter().map(| f | f.to_string()).collect
        :: < Vec < String >> (); if let Some(web) = & web
        {
            i += 1; insert_fields.push("web");
            insert_values.push(format! ("${}", i));
        } if let Some(linkedin) = & linkedin
        {
            i += 1; insert_fields.push("linkedin");
            insert_values.push(format! ("${}", i));
        } if let Some(github) = & github
        {
            i += 1; insert_fields.push("github");
            insert_values.push(format! ("${}", i));
        } let query = format!
        ("INSERT INTO members ({}) VALUES ({}) RETURNING id, created_at, updated_at, name, image, role, email, web, linkedin, github, description",
        insert_fields.join(", "), insert_values.join(", "),); tracing ::
        trace! ("insert query: {}", query); let mut q = sqlx ::
        query(&
        query).bind(name).bind(image).bind(role).bind(email).bind(description);
        if let Some(web) = & web { q = q.bind(web); } if let Some(linkedin) =
        & linkedin { q = q.bind(linkedin); } if let Some(github) = & github
        { q = q.bind(github); } let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Member
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), name :
                row.try_get("name").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), role :
                row.try_get("role").unwrap_or_default(), email :
                row.try_get("email").unwrap_or_default(), web :
                row.try_get("web").unwrap_or_default(), linkedin :
                row.try_get("linkedin").unwrap_or_default(), github :
                row.try_get("github").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn insert_with_tx < 'e, 'c : 'e, E >
    (& self, tx : E, name : String, image : String, role : MemberRole, email :
    String, web : Option < String > , linkedin : Option < String > , github :
    Option < String > , description : String) -> crate::Result < Option <
    Member >> where E : sqlx :: Executor < 'c, Database = sqlx :: postgres ::
    Postgres > ,
    {
        let mut i = 5; let mut insert_fields = vec!
        ["name", "image", "role", "email", "description"]; let mut
        insert_values = vec!
        ["$1", "$2", "$3", "$4", "$5"].iter().map(| f | f.to_string()).collect
        :: < Vec < String >> (); if let Some(web) = & web
        {
            i += 1; insert_fields.push("web");
            insert_values.push(format! ("${}", i));
        } if let Some(linkedin) = & linkedin
        {
            i += 1; insert_fields.push("linkedin");
            insert_values.push(format! ("${}", i));
        } if let Some(github) = & github
        {
            i += 1; insert_fields.push("github");
            insert_values.push(format! ("${}", i));
        } let query = format!
        ("INSERT INTO members ({}) VALUES ({}) RETURNING id, created_at, updated_at, name, image, role, email, web, linkedin, github, description",
        insert_fields.join(", "), insert_values.join(", "),); tracing ::
        trace! ("insert query: {}", query); let mut q = sqlx ::
        query(&
        query).bind(name).bind(image).bind(role).bind(email).bind(description);
        if let Some(web) = & web { q = q.bind(web); } if let Some(linkedin) =
        & linkedin { q = q.bind(linkedin); } if let Some(github) = & github
        { q = q.bind(github); } let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Member
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), name :
                row.try_get("name").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), role :
                row.try_get("role").unwrap_or_default(), email :
                row.try_get("email").unwrap_or_default(), web :
                row.try_get("web").unwrap_or_default(), linkedin :
                row.try_get("linkedin").unwrap_or_default(), github :
                row.try_get("github").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default()
            }
        }).fetch_optional(tx).await ? ; Ok(row)
    } pub async fn
    insert_without_returning(& self, name : String, image : String, role :
    MemberRole, email : String, web : Option < String > , linkedin : Option <
    String > , github : Option < String > , description : String) ->
    crate::Result < () >
    {
        let mut i = 5; let mut insert_fields = vec!
        ["name", "image", "role", "email", "description"]; let mut
        insert_values = vec!
        ["$1", "$2", "$3", "$4", "$5"].iter().map(| f | f.to_string()).collect
        :: < Vec < String >> (); if let Some(web) = & web
        {
            i += 1; insert_fields.push("web");
            insert_values.push(format! ("${}", i));
        } if let Some(linkedin) = & linkedin
        {
            i += 1; insert_fields.push("linkedin");
            insert_values.push(format! ("${}", i));
        } if let Some(github) = & github
        {
            i += 1; insert_fields.push("github");
            insert_values.push(format! ("${}", i));
        } let query = format!
        ("INSERT INTO members ({}) VALUES ({})", insert_fields.join(", "),
        insert_values.join(", "),); tracing :: trace!
        ("insert query: {}", query); let mut q = sqlx ::
        query(&
        query).bind(name).bind(image).bind(role).bind(email).bind(description);
        if let Some(web) = & web { q = q.bind(web); } if let Some(linkedin) =
        & linkedin { q = q.bind(linkedin); } if let Some(github) = & github
        { q = q.bind(github); } q.execute(& self.pool).await ? ; Ok(())
    } pub async fn
    update(& self, id : i64, member_repository_update_request :
    MemberRepositoryUpdateRequest) -> crate::Result < Member >
    {
        let mut i = 1; let mut update_values = vec! []; if
        member_repository_update_request.name.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "name", i)); } if
        member_repository_update_request.image.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "image", i)); } if
        member_repository_update_request.role.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "role", i)); } if
        member_repository_update_request.email.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "email", i)); } if
        member_repository_update_request.web.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "web", i)); } if
        member_repository_update_request.linkedin.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "linkedin", i)); }
        if member_repository_update_request.github.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "github", i)); } if
        member_repository_update_request.description.is_some()
        {
            i += 1;
            update_values.push(format! ("{} = ${}", "description", i));
        } let query = format!
        ("UPDATE members SET {} WHERE id = $1 RETURNING id, created_at, updated_at, name, image, role, email, web, linkedin, github, description",
        update_values.join(", "),); tracing :: trace!
        ("insert query: {}", query); let mut q = sqlx ::
        query(& query).bind(id); if let Some(name) =
        member_repository_update_request.name { q = q.bind(name); } if let
        Some(image) = member_repository_update_request.image
        { q = q.bind(image); } if let Some(role) =
        member_repository_update_request.role { q = q.bind(role); } if let
        Some(email) = member_repository_update_request.email
        { q = q.bind(email); } if let Some(web) =
        member_repository_update_request.web { q = q.bind(web); } if let
        Some(linkedin) = member_repository_update_request.linkedin
        { q = q.bind(linkedin); } if let Some(github) =
        member_repository_update_request.github { q = q.bind(github); } if let
        Some(description) = member_repository_update_request.description
        { q = q.bind(description); } let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Member
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), name :
                row.try_get("name").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), role :
                row.try_get("role").unwrap_or_default(), email :
                row.try_get("email").unwrap_or_default(), web :
                row.try_get("web").unwrap_or_default(), linkedin :
                row.try_get("linkedin").unwrap_or_default(), github :
                row.try_get("github").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn update_with_tx < 'e, 'c : 'e, E >
    (& self, tx : E, id : i64, member_repository_update_request :
    MemberRepositoryUpdateRequest) -> crate::Result < Option < Member >> where
    E : sqlx :: Executor < 'c, Database = sqlx :: postgres :: Postgres > ,
    {
        let mut i = 1; let mut update_values = vec! []; if
        member_repository_update_request.name.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "name", i)); } if
        member_repository_update_request.image.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "image", i)); } if
        member_repository_update_request.role.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "role", i)); } if
        member_repository_update_request.email.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "email", i)); } if
        member_repository_update_request.web.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "web", i)); } if
        member_repository_update_request.linkedin.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "linkedin", i)); }
        if member_repository_update_request.github.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "github", i)); } if
        member_repository_update_request.description.is_some()
        {
            i += 1;
            update_values.push(format! ("{} = ${}", "description", i));
        } let query = format!
        ("UPDATE members SET {} WHERE id = $1 RETURNING id, created_at, updated_at, name, image, role, email, web, linkedin, github, description",
        update_values.join(", "),); tracing :: trace!
        ("insert query: {}", query); let mut q = sqlx ::
        query(& query).bind(id); if let Some(name) =
        member_repository_update_request.name { q = q.bind(name); } if let
        Some(image) = member_repository_update_request.image
        { q = q.bind(image); } if let Some(role) =
        member_repository_update_request.role { q = q.bind(role); } if let
        Some(email) = member_repository_update_request.email
        { q = q.bind(email); } if let Some(web) =
        member_repository_update_request.web { q = q.bind(web); } if let
        Some(linkedin) = member_repository_update_request.linkedin
        { q = q.bind(linkedin); } if let Some(github) =
        member_repository_update_request.github { q = q.bind(github); } if let
        Some(description) = member_repository_update_request.description
        { q = q.bind(description); } let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Member
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), name :
                row.try_get("name").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), role :
                row.try_get("role").unwrap_or_default(), email :
                row.try_get("email").unwrap_or_default(), web :
                row.try_get("web").unwrap_or_default(), linkedin :
                row.try_get("linkedin").unwrap_or_default(), github :
                row.try_get("github").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default()
            }
        }).fetch_optional(tx).await ? ; Ok(row)
    } pub async fn delete(& self, id : i64) -> crate::Result < Member >
    {
        let res = sqlx ::
        query("DELETE FROM members WHERE id = $1 RETURNING *").bind(id).map(Member
        :: from).fetch_one(& self.pool).await ? ; Ok(res)
    } pub async fn delete_with_tx < 'e, 'c : 'e, E >
    (& self, tx : E, id : i64) -> crate::Result < Option < Member >> where E :
    sqlx :: Executor < 'c, Database = sqlx :: postgres :: Postgres > ,
    {
        let res = sqlx ::
        query("DELETE FROM members WHERE id = $1 RETURNING *").bind(id).map(Member
        :: from).fetch_optional(tx).await ? ; Ok(res)
    } pub async fn find_one(& self, param : & MemberReadAction) ->
    crate::Result < Member >
    {
        let mut query = format! ("{}", Member :: base_sql());
        query.push_str(" "); query.push_str(Member :: group_by().as_str());
        tracing :: trace!
        ("{} query {}: {:?}", "MemberRepository::find_one", query, param); let
        mut q = sqlx :: query(& query); let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Member
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), name :
                row.try_get("name").unwrap_or_default(), image :
                row.try_get("image").unwrap_or_default(), role :
                row.try_get("role").unwrap_or_default(), email :
                row.try_get("email").unwrap_or_default(), web :
                row.try_get("web").unwrap_or_default(), linkedin :
                row.try_get("linkedin").unwrap_or_default(), github :
                row.try_get("github").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn find(& self, param : & MemberQuery) -> crate::Result <
    by_types::QueryResponse<MemberSummary> >
    {
        let query = format!
        ("WITH data AS ({} {}) SELECT ({}) AS total_count, data.* FROM data;",
        "SELECT * FROM members", "LIMIT $1 OFFSET $2",
        "SELECT COUNT(*) FROM members"); tracing :: trace!
        ("{} query {}", "MemberRepository::find_one", query); let offset : i32
        = (param.size as i32) * (param.page() - 1); let q = sqlx ::
        query(& query).bind(param.size as i32).bind(offset); let mut total :
        i64 = 0; let rows =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; total = row.get("total_count"); row.into()
        }).fetch_all(& self.pool).await ? ; Ok((rows, total).into())
    }
} impl From < sqlx :: postgres :: PgRow > for Member
{
    fn from(row : sqlx :: postgres :: PgRow) -> Self
    {
        use sqlx :: Row; Member
        {
            id : row.try_get("id").unwrap_or_default(), created_at :
            row.try_get("created_at").unwrap_or_default(), updated_at :
            row.try_get("updated_at").unwrap_or_default(), name :
            row.try_get("name").unwrap_or_default(), image :
            row.try_get("image").unwrap_or_default(), role :
            row.try_get("role").unwrap_or_default(), email :
            row.try_get("email").unwrap_or_default(), web :
            row.try_get("web").unwrap_or_default(), linkedin :
            row.try_get("linkedin").unwrap_or_default(), github :
            row.try_get("github").unwrap_or_default(), description :
            row.try_get("description").unwrap_or_default()
        }
    }
} impl From < sqlx :: postgres :: PgRow > for MemberSummary
{
    fn from(row : sqlx :: postgres :: PgRow) -> Self
    {
        use sqlx :: Row; MemberSummary
        {
            id : row.try_get("id").unwrap_or_default(), created_at :
            row.try_get("created_at").unwrap_or_default(), updated_at :
            row.try_get("updated_at").unwrap_or_default(), name :
            row.try_get("name").unwrap_or_default(), image :
            row.try_get("image").unwrap_or_default(), role :
            row.try_get("role").unwrap_or_default(), email :
            row.try_get("email").unwrap_or_default(), web :
            row.try_get("web").unwrap_or_default(), linkedin :
            row.try_get("linkedin").unwrap_or_default(), github :
            row.try_get("github").unwrap_or_default(), description :
            row.try_get("description").unwrap_or_default()
        }
    }
} #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default)]
pub struct MemberRepositoryQueryBuilder
{
    pub base_sql : String, pub group_by : String, pub count : bool, pub
    conditions : Vec < by_types :: Conditions > , pub order : by_types ::
    Order, pub limit : Option < i32 > , pub page : Option < i32 > , pub or :
    Vec < Vec < by_types :: Conditions >> ,
} impl std :: ops :: BitOr for MemberRepositoryQueryBuilder
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
} impl std :: ops :: BitOrAssign for MemberRepositoryQueryBuilder
{
    fn bitor_assign(& mut self, rhs : Self)
    {
        self.or.push(rhs.conditions); if ! rhs.or.is_empty()
        { self.or.extend(rhs.or); } self.count = self.count || rhs.count;
        self.limit = self.limit.or(rhs.limit); self.page =
        self.page.or(rhs.page);
    }
} impl MemberRepositoryQueryBuilder
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
        ("Building where clause for {}", "MemberRepositoryQueryBuilder"); let
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
    } pub fn name_equals(mut self, name : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("name".to_string(), name)); self
    } pub fn name_not_equals(mut self, name : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("name".to_string(), name)); self
    } pub fn name_contains(mut self, name : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("name".to_string(), name)); self
    } pub fn name_not_contains(mut self, name : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("name".to_string(), name)); self
    } pub fn name_starts_with(mut self, name : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("name".to_string(), name)); self
    } pub fn name_not_starts_with(mut self, name : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("name".to_string(), name)); self
    } pub fn name_ends_with(mut self, name : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("name".to_string(), name)); self
    } pub fn name_not_ends_with(mut self, name : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("name".to_string(), name)); self
    } pub fn order_by_name_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "name")); } else
        { self.order = by_types :: Order :: Asc(vec! ["name".to_string()]); }
        self
    } pub fn order_by_name_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "name")); } else
        { self.order = by_types :: Order :: Desc(vec! ["name".to_string()]); }
        self
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
    } pub fn role_equals(mut self, role : MemberRole) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsInteger("role".to_string(), role.into())); self
    } pub fn role_not_equals(mut self, role : MemberRole) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsInteger("role".to_string(), role.into())); self
    } pub fn role_greater_than(mut self, role : MemberRole) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanInteger("role".to_string(), role.into())); self
    } pub fn role_less_than(mut self, role : MemberRole) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanInteger("role".to_string(), role.into())); self
    } pub fn role_greater_than_equals(mut self, role : MemberRole) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanEqualsInteger("role".to_string(), role.into())); self
    } pub fn role_less_than_equals(mut self, role : MemberRole) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanEqualsInteger("role".to_string(), role.into())); self
    } pub fn role_between(mut self, from : MemberRole, to : MemberRole) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        BetweenInteger("role".to_string(), from.into(), to.into())); self
    } pub fn order_by_role_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "role")); } else
        { self.order = by_types :: Order :: Asc(vec! ["role".to_string()]); }
        self
    } pub fn order_by_role_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "role")); } else
        { self.order = by_types :: Order :: Desc(vec! ["role".to_string()]); }
        self
    } pub fn email_equals(mut self, email : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("email".to_string(), email)); self
    } pub fn email_not_equals(mut self, email : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("email".to_string(), email)); self
    } pub fn email_contains(mut self, email : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("email".to_string(), email)); self
    } pub fn email_not_contains(mut self, email : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("email".to_string(), email)); self
    } pub fn email_starts_with(mut self, email : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("email".to_string(), email)); self
    } pub fn email_not_starts_with(mut self, email : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("email".to_string(), email)); self
    } pub fn email_ends_with(mut self, email : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("email".to_string(), email)); self
    } pub fn email_not_ends_with(mut self, email : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("email".to_string(), email)); self
    } pub fn order_by_email_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "email")); } else
        { self.order = by_types :: Order :: Asc(vec! ["email".to_string()]); }
        self
    } pub fn order_by_email_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "email")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["email".to_string()]);
        } self
    } pub fn web_equals(mut self, web : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("web".to_string(), web)); self
    } pub fn web_not_equals(mut self, web : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("web".to_string(), web)); self
    } pub fn web_contains(mut self, web : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("web".to_string(), web)); self
    } pub fn web_not_contains(mut self, web : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("web".to_string(), web)); self
    } pub fn web_starts_with(mut self, web : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("web".to_string(), web)); self
    } pub fn web_not_starts_with(mut self, web : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("web".to_string(), web)); self
    } pub fn web_ends_with(mut self, web : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("web".to_string(), web)); self
    } pub fn web_not_ends_with(mut self, web : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("web".to_string(), web)); self
    } pub fn order_by_web_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "web")); } else
        { self.order = by_types :: Order :: Asc(vec! ["web".to_string()]); }
        self
    } pub fn order_by_web_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "web")); } else
        { self.order = by_types :: Order :: Desc(vec! ["web".to_string()]); }
        self
    } pub fn linkedin_equals(mut self, linkedin : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("linkedin".to_string(), linkedin)); self
    } pub fn linkedin_not_equals(mut self, linkedin : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("linkedin".to_string(), linkedin)); self
    } pub fn linkedin_contains(mut self, linkedin : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("linkedin".to_string(), linkedin)); self
    } pub fn linkedin_not_contains(mut self, linkedin : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("linkedin".to_string(), linkedin)); self
    } pub fn linkedin_starts_with(mut self, linkedin : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("linkedin".to_string(), linkedin)); self
    } pub fn linkedin_not_starts_with(mut self, linkedin : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("linkedin".to_string(), linkedin)); self
    } pub fn linkedin_ends_with(mut self, linkedin : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("linkedin".to_string(), linkedin)); self
    } pub fn linkedin_not_ends_with(mut self, linkedin : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("linkedin".to_string(), linkedin)); self
    } pub fn order_by_linkedin_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "linkedin")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["linkedin".to_string()]);
        } self
    } pub fn order_by_linkedin_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "linkedin")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["linkedin".to_string()]);
        } self
    } pub fn github_equals(mut self, github : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("github".to_string(), github)); self
    } pub fn github_not_equals(mut self, github : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("github".to_string(), github)); self
    } pub fn github_contains(mut self, github : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("github".to_string(), github)); self
    } pub fn github_not_contains(mut self, github : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("github".to_string(), github)); self
    } pub fn github_starts_with(mut self, github : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("github".to_string(), github)); self
    } pub fn github_not_starts_with(mut self, github : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("github".to_string(), github)); self
    } pub fn github_ends_with(mut self, github : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("github".to_string(), github)); self
    } pub fn github_not_ends_with(mut self, github : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("github".to_string(), github)); self
    } pub fn order_by_github_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "github")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["github".to_string()]);
        } self
    } pub fn order_by_github_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "github")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["github".to_string()]);
        } self
    } pub fn description_equals(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("description".to_string(), description)); self
    } pub fn description_not_equals(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("description".to_string(), description)); self
    } pub fn description_contains(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("description".to_string(), description)); self
    } pub fn description_not_contains(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("description".to_string(), description)); self
    } pub fn description_starts_with(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("description".to_string(), description)); self
    } pub fn description_not_starts_with(mut self, description : String) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("description".to_string(), description)); self
    } pub fn description_ends_with(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("description".to_string(), description)); self
    } pub fn description_not_ends_with(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("description".to_string(), description)); self
    } pub fn order_by_description_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "description")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["description".to_string()]);
        } self
    } pub fn order_by_description_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "description")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["description".to_string()]);
        } self
    } pub fn order_by_random(mut self) -> Self
    { self.order = by_types :: Order :: Random; self }
} /// Member is a generated struct that represents the model
///
/// For making API calls related to this model, use `Member::get_client(endpoint: &str)`.
/// It will returns MemberClient struct that implements the API calls.
///
/// In server side, you can use `Member::get_repository()` to interact with the database.
/// Recommend to use `MemberRepository` to insert or update the model.
/// To query the model, use `Member::query_builder()`.
/// For more detail, refer to the documentation of the query builder.
#[derive(Debug, Clone, serde :: Deserialize, serde :: Serialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct Member
{
    pub id : i64, pub created_at : i64, pub updated_at : i64, pub name :
    String, pub image : String, pub role : MemberRole, pub email : String, pub
    web : Option < String > , pub linkedin : Option < String > , pub github :
    Option < String > , pub description : String
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo, sqlx :: FromRow))] pub
struct MemberSummary
{
    pub id : i64, pub created_at : i64, pub updated_at : i64, pub name :
    String, pub image : String, pub role : MemberRole, pub email : String, pub
    web : Option < String > , pub linkedin : Option < String > , pub github :
    Option < String > , pub description : String,
} impl From < Member > for MemberSummary
{
    fn from(item : Member) -> Self
    {
        Self
        {
            id : item.id, created_at : item.created_at, updated_at :
            item.updated_at, name : item.name, image : item.image, role :
            item.role, email : item.email, web : item.web, linkedin :
            item.linkedin, github : item.github, description :
            item.description,
        }
    }
} impl Into < Member > for MemberSummary
{
    fn into(self) -> Member
    {
        Member
        {
            id : self.id, created_at : self.created_at, updated_at :
            self.updated_at, name : self.name, image : self.image, role :
            self.role, email : self.email, web : self.web, linkedin :
            self.linkedin, github : self.github, description :
            self.description, .. Default :: default()
        }
    }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq, by_macros :: QueryDisplay)] #[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct MemberQuery
{
    #[serde(deserialize_with = "parse_size_of_member_query", default)] pub
    size : usize, pub bookmark : Option < String > ,
} pub fn parse_size_of_member_query < 'de, D > (deserializer : D) -> std ::
result :: Result < usize, D :: Error > where D : serde :: Deserializer < 'de >
,
{
    use serde :: Deserialize; let s : Option < String > = Option ::
    deserialize(deserializer) ? ;
    s.unwrap_or_else(|| Default :: default()).parse :: < usize >
    ().map_err(serde :: de :: Error :: custom)
} impl MemberQuery
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
} impl MemberClient {} impl Member
{
    pub fn get_client(endpoint : & str) -> MemberClient
    { MemberClient { endpoint : endpoint.to_string() } }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)] pub struct MemberClient { pub endpoint : String, } impl
MemberClient
{
    pub async fn query(& self, params : MemberQuery,) -> crate::Result <
    by_types::QueryResponse<MemberSummary> >
    {
        let path = format! ("/v1/members",); let endpoint = format!
        ("{}{}", self.endpoint, path); let query = format!
        ("{}?{}", endpoint, MemberParam :: Query(params)); rest_api ::
        get(& query).await
    } pub async fn get(& self, id : i64) -> crate::Result < Member >
    {
        let path = format! ("/v1/members",); let endpoint = format!
        ("{}{}/{}", self.endpoint, path, id); rest_api ::
        get(& endpoint).await
    }
} impl Member { pub fn url() -> String { "/v1/members".to_string() } }
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq, by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
MemberReadAction {} impl MemberReadAction
{ pub fn new() -> Self { Self :: default() } } impl MemberClient {}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, PartialEq,
by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server", derive(aide :: OperationIo))]
#[serde(tag = "param-type", rename_all = "kebab-case")] pub enum MemberParam
{ Query(MemberQuery), } #[cfg(feature = "server")] impl schemars :: JsonSchema
for MemberParam
{
    fn schema_name() -> String { "MemberParam".to_string() } fn
    json_schema(_gen : & mut schemars :: gen :: SchemaGenerator) -> schemars
    :: schema :: Schema
    {
        let mut schema_obj = schemars :: schema :: SchemaObject :: default();
        schema_obj.metadata =
        Some(Box ::
        new(schemars :: schema :: Metadata
        {
            title : Some("Member Query Parameters".to_string()), .. Default ::
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
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum
MemberGetResponse { Query(by_types::QueryResponse<MemberSummary>), }