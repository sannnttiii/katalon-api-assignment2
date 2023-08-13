<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-Post Sort by ID in Descending Order</name>
   <tag></tag>
   <elementGuidId>e5d757f1-6f1a-492f-ae83-fedc73ff4f78</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/posts?_sort=id&amp;_order=desc</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, '[0].userId', 10)
WS.verifyElementPropertyValue(response, '[0].id', 100)
WS.verifyElementPropertyValue(response, '[0].title', 'at nam consequatur ea labore ea harum')
WS.verifyElementPropertyValue(response, '[0].body', 'cupiditate quo est a modi nesciunt soluta\nipsa voluptas error itaque dicta in\nautem qui minus magnam et distinctio eum\naccusamus ratione error aut')
WS.verifyElementPropertyValue(response, '[1].userId', 10)
WS.verifyElementPropertyValue(response, '[1].id', 99)
WS.verifyElementPropertyValue(response, '[1].title', 'temporibus sit alias delectus eligendi possimus magni')
WS.verifyElementPropertyValue(response, '[1].body', 'quo deleniti praesentium dicta non quod\naut est molestias\nmolestias et officia quis nihil\nitaque dolorem quia')
WS.verifyElementPropertyValue(response, '[2].userId', 10)
WS.verifyElementPropertyValue(response, '[2].id', 98)
WS.verifyElementPropertyValue(response, '[2].title', 'laboriosam dolor voluptates')
WS.verifyElementPropertyValue(response, '[2].body', 'doloremque ex facilis sit sint culpa\nsoluta assumenda eligendi non ut eius\nsequi ducimus vel quasi\nveritatis est dolores')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
