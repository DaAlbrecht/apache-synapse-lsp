# Log Mediator

The **Log mediator** is used to log mediated messages. For more information on logging, see [Monitoring Logs]({{base_path}}/observe/micro-integrator/classic-observability-logs/monitoring-logs/).

## Syntax

The log token refers to a `<log>` element, which may be
used to log messages being mediated.

```java
<log [level="string"] [separator="string"]>
   <property name="string" (value="literal" | expression="[XPath|json-eval(JSON Path)]")/>*
</log>

```
