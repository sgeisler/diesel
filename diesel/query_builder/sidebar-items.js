initSidebarItems({"mod":[["bind_collector",""],["debug",""],["insert_statement",""],["update_statement",""],["where_clause",""]],"struct":[["IncompleteDefaultInsertStatement","The structure returned by `insert_default_values`. The only thing that can be done with it is call `into`."],["IncompleteInsertStatement","The structure returned by `insert`. The only thing that can be done with it is call `into`."],["IncompleteUpdateStatement","The type returned by `update`. The only thing you can do with this type is call `set` on it."],["UpdateStatement",""]],"trait":[["AsChangeset","Types which can be passed to `update.set`."],["AsQuery","Types that can be converted into a complete, typed SQL query. This is used internally to automatically add the right select clause when none is specified, or to automatically add `RETURNING *` in certain contexts"],["Changeset","Apps should not need to concern themselves with this trait."],["IntoUpdateTarget","A type which can be passed to `update`."],["Query","A complete SQL query with a return type. This can be a select statement, or a command such as `update` or `insert` with a `RETURNING` clause. Unlike `Expression`, types implementing this trait are guaranteed to be executable on their own."],["QueryBuilder","Apps should not need to concern themselves with this trait."],["QueryFragment","An untyped fragment of SQL. This may be a complete SQL command (such as an update statement without a `RETURNING` clause), or a subsection (such as our internal types used to represent a `WHERE` clause). All methods on `Connection` that execute a query require this trait to be implemented."],["QueryId",""]],"type":[["BuildQueryResult",""]]});