# Log Mediator

The **Log mediator** is used to log mediated messages. For more information on logging, see [Monitoring Logs](https://github.com/wso2/docs-apim/blob/4.2.0/en/docs/reference/mediators/log-mediator.md).

---

## Syntax

The log token refers to a `<log>` element, which may be
used to log messages being mediated.

```java
<log [level="string"] [separator="string"]>
   <property name="string" (value="literal" | expression="[XPath|json-eval(JSON Path)]")/>*
</log>
```
