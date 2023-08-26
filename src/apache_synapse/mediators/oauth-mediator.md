# OAuth Mediator

The **OAuth Mediator** supports 2 forms of OAuth. It bypasses the RESTful requests and authenticates users against WSO2 Identity Server.

When a client tries to invoke a RESTful service, it may be required to verify the credentials of the client. This can be achieved by registering an OAuth application in the WSO2 Identity Server. When the client sends a REST call with the Authorization header to the Micro Integrator, the OAuth mediator validates it with the Identity server and proceeds.

See [2-legged OAuth for Securing a RESTful Service](https://docs.wso2.com/display/IS570/2-legged+OAuth+for+Securing+a+RESTful+Service) for detailed instructions to carry out this process.

---

## Info

If you are using OAuth 1 a, you will get the `org.apache.synapse.SynapseException: Unable to find SCOPE value in Synapse Message Context` error when the `SCOPE` property is not set in the synapse message context. To avoid this error, add a property with the name `scope` and a value in the synapse message context as shown in the [Example](#example) section.

## Syntax

``` java
<oauthService remoteServiceUrl="" username="" password=""/>
```
