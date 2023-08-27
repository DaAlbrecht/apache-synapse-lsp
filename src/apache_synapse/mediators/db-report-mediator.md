# DB Report Mediator

The **DB Report Mediator** is similar to the [DBLookup Mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/dblookup-mediator/). The difference between the two mediators is that the DB Report mediator writes information to a database using the specified insert SQL statement.

---

##  Info

The DB Report mediator is a [content-aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

Currently, the 'DB-Report-mediator' does not support the 'json-eval' expression used to extract the parameters.

## Syntax

The syntax of the DB Report mediator changes depending on whether you connect to the database using a connection pool, or using a data source.

-   **Connection Pool**

``` java
<dbreport>
   <connection>
     <pool>
      (
        <driver/>
        <url/>
        <user/>
        <password/>

        <dsName/>
        <icClass/>
        <url/>
        <user/>
        <password/>
      )
        <property name="name" value="value"/>*
     </pool>
   </connection>
   <statement>
       <sql>insert into something values(?, ?, ?, ?)</sql>
      <parameter [value="" | expression=""] type="CHAR|VARCHAR|LONGVARCHAR|NUMERIC|DECIMAL|BIT|TINYINT|SMALLINT|INTEGER|BIGINT|REAL|FLOAT|DOUBLE|DATE|TIME|TIMESTAMP"/>*
   </statement>+
</dbreport>
```

-   **Data source**

    The syntax of the DBLookup mediator further differs based on whether the connection to the database is made using an external datasource or a Carbon datasource. Click on the relevant tab to view the required syntax.

    ``` java tab='External Datasource'
    <dbreport>
       <connection>
          <pool>
            <dsName/>
            <icClass/>
            <url/>
            <user/>
            <password/>
            <property name="name" value="value"/>*
          </pool>
       </connection>
       <statement>
          <sql>select something from table where something_else = ?</sql>
          <parameter [value="" | expression=""] type="CHAR|VARCHAR|LONGVARCHAR|NUMERIC|DECIMAL|BIT|TINYINT|SMALLINT|INTEGER|BIGINT|REAL|FLOAT|DOUBLE|DATE|TIME|TIMESTAMP"/>*
       </statement>+
    </dbreport>
    ```

    ``` java tab='Carbon Datasource'
    <dbreport>
       <connection>
          <pool>
            <dsName/>
          </pool>
       </connection>
       <statement>
          <sql>select something from table where something_else = ?</sql>
          <parameter [value="" | expression=""] type="CHAR|VARCHAR|LONGVARCHAR|NUMERIC|DECIMAL|BIT|TINYINT|SMALLINT|INTEGER|BIGINT|REAL|FLOAT|DOUBLE|DATE|TIME|TIMESTAMP"/>*
       </statement>+
    </dbreport>
    ```
