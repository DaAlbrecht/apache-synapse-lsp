# Call Mediator

The **Call mediator** is used to send messages out of the Micro Integrator to an **endpoint**. You can invoke services either in blocking or non-blocking manner.

## Syntax

``` java
<call [blocking="true|false"]>
   <source contentType=" " type="custom|inline|property">{xpath|inline|property}</source>?
   <target type=”property”>{property_name}</target>?
   (endpointref | endpoint)
</call>
```
