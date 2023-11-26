# Fault Mediator

The **Fault Mediator** (also called the **Makefault Mediator**) transforms the current message into a fault message. However, this
mediator does not send the converted message. The [Send Mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/send-mediator/) needs to be invoked to send a fault message
created via the Fault mediator. The fault message's `To` header is set to the `Fault-To` of the original message (if such a header exists in the original message). You can create the fault message as a SOAP 1.1, SOAP 1.2, or plain-old XML (POX) fault.

For more information on faults and errors, see [Error Handling]({{base_path}}/reference/error_handling).

---

## Syntax

``` java
<makefault [version="soap11|soap12|pox"]>
<code (value="literal" | expression="xpath")/>
<reason (value="literal" | expression="xpath")>
<node>?
<role>?
<detail>?
</makefault>
```
