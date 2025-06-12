Creating a backend for an analytical tool - hobby project
Plans:
  - Authentication
  - Data Sources:
      - Allow multiple independent table from different connections. Live connections / extracts
      - Allow joins / unions to construct expanded table of multiple tables. Each independent table can have this feature.
      - Allow calculations into a table from other tables. Calculations and results are to be cached if connection is extract.
      - Allow independent variables calculation that are not on table level, stored seperately. E.g. arrays (like distinct field values used for other calculations) or Sums
      - Database, CSV, Parquet connections (for now)
  - Data Processing:
      - Tied to Data Sources.
      - Polars will handle the calculations.
  - Visualization Pipeline to be defined. The above Data Processing and Data Sources should be flexible enough for this to be decided later. For now some ideas:
        - Pivoting for visualizations from tables.
        - Allow the independently stored variables to be queried if needed to visualized.
        - If truly independent and need to be on a chart, have to introduce multiple axis, synced axis. Everything depends on what will be the visualization library.