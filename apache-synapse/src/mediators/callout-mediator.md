# Callout Mediator

The **Callout** mediator performs a blocking external service invocation during mediation. As the Callout mediator performs a blocking call, it cannot use the default non-blocking HTTP/S transports based on Java NIO.

---

## Tip

The [Call mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/call-mediator/) leverages the non-blocking transports for much greater performance than the Callout mediator. Therefore, you should use the Call mediator in most cases. However, the Callout mediator is recommended in situations where you need to execute the mediation flow in a single thread.

## Enabling mutual SSL

The Callout mediators default https transport sender is `org.apache.axis2.transport.http.CommonsHTTPTransportSender`. Therefore, the Callout mediator does not have access to the required key store to handle mutual SSL. To enable the Callout mediator to handle mutual SSL, the following JVM settings should be added to the `MI_HOME/bin/micro-integrator.sh` file.

```
-Djavax.net.ssl.keyStore="$CARBON_HOME/repository/resources/security/wso2carbon.jks" \
-Djavax.net.ssl.keyStorePassword="wso2carbon" \
-Djavax.net.ssl.keyPassword="wso2carbon" \
```

## Disabling chunking

The Callout mediator is not affected by the [DISABLE_CHUNKING property](https://apim.docs.wso2.com/en/latest/reference/mediators/property-reference/http-transport-properties/). Instead, you can disable chunking for the Callout mediator by setting the following parameters in the `MI_HOME/conf/deployment.toml` file:

```toml
[transport.blocking.http]
sender.transfer_encoding = "chunked"
```

This will disable chunking for all Callout mediators present in the Micro Integrator.

If you want to disable chunking for only a single Callout mediator instance, create a new `axis2.xml` file by copying the `MI_HOME/conf/axis2/axis2_blocking_client.xml` file, set the `Transfer-Encoding` parameter as shown, and then configure that Callout mediator to use this new `axis2.xml` file as described below.

## Syntax

``` java
<callout [serviceURL="string"] [action="string"] [initAxis2ClientOptions="true|false"] [endpointKey="string"]>
      <configuration [axis2xml="string"] [repository="string"]/>?
      <source xpath="expression" | key="string" | type="envelope"/>
      <target xpath="expression" | key="string"/>
      <enableSec policy="string" | outboundPolicy="String" | inboundPolicy="String" />?
</callout>
```
