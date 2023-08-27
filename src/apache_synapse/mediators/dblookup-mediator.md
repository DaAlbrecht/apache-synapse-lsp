# DBLookup Mediator

The **DBLookup Mediator** can execute an arbitrary SQL select statement
and then set a resulting values as a local message property in the
message context. The DB connection used may be looked up from an
external data source or specified inline.

---

## Info

- The DBLookup mediator is a [content-aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.
- The DBLookup mediator can set a property from one row in a result set. It cannot return multiple rows. If you need to get multiple records, or if you have a table with multiple parameters (such as URLs), you can create a data service and invoke that service from the Micro Integrator using the [Callout mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/callout-mediator/) instead.

## Syntax

The syntax of the DBLookup mediator changes depending on whether you connect to the database using a connection pool, or using a data source. Click on the relevant tab to view the required syntax.

-   **Connection Pool**

``` java
<dblookup>
   <connection>
      <pool>
        <driver/>
        <url/>
        <user/>
        <password/>
        <property name="name" value="value"/>*
      </pool>
   </connection>
   <statement>
      <sql>select something from table where something_else = ?</sql>
      <parameter [value="" | expression=""] type="CHAR|VARCHAR|LONGVARCHAR|NUMERIC|DECIMAL|BIT|TINYINT|SMALLINT|INTEGER|BIGINT|REAL|FLOAT|DOUBLE|DATE|TIME|TIMESTAMP"/>*
      <result name="string" column="int|string"/>*
   </statement>+
</dblookup>
```

-   **Data source**
    The syntax of the DBLookup mediator further differs based on whether the connection to the database is made using an external datasource or a Carbon datasource.

    ``` java tab='External Datasource'
    <dblookup>
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
          <result name="string" column="int|string"/>*
       </statement>+
    </dblookup>
    ```

    ``` java tab='Carbon Datasource'
    <dblookup>
       <connection>
          <pool>
            <dsName/>
          </pool>
       </connection>
       <statement>
          <sql>select something from table where something_else = ?</sql>
          <parameter [value="" | expression=""] type="CHAR|VARCHAR|LONGVARCHAR|NUMERIC|DECIMAL|BIT|TINYINT|SMALLINT|INTEGER|BIGINT|REAL|FLOAT|DOUBLE|DATE|TIME|TIMESTAMP"/>*
          <result name="string" column="int|string"/>*
       </statement>+
    </dblookup>
    ```
