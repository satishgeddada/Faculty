<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For saving single copo item record</description>
   <name>Saving single copo item record</name>
   <tag></tag>
   <elementGuidId>2d882d7c-700f-47c6-a4a7-9075d3a75dca</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${URL}/faculty/CourseCoordinatorServlet/ForSaveCoPo?copo=%7B%22coName%22%3A%22Understanding%20the%20concept%20of%20therory%20practice%20with%20valid%20resons%22%2C%22status%22%3A%22No%20DataAvaliabale%22%2C%22coTemp%22%3A4%2C%22copo1%22%3A%221%22%2C%22copo2%22%3A%221%22%2C%22copo3%22%3A%22NIL%22%2C%22copo4%22%3A%22NIL%22%2C%22copo5%22%3A%22NIL%22%2C%22copo6%22%3A%22NIL%22%2C%22copo7%22%3A%221%22%2C%22copo8%22%3A%221%22%2C%22copo9%22%3A%221%22%2C%22copo10%22%3A%221%22%2C%22copo11%22%3A%221%22%2C%22copo12%22%3A%221%22%2C%22copo13%22%3A%22NIL%22%2C%22copo14%22%3A%22NIL%22%2C%22copo15%22%3A%221%22%7D&amp;co=Understanding%20the%20concept%20of%20therory%20practice%20with%20valid%20resons&amp;course=R17-7G134&amp;options=ForSaveCoPo</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.URL</defaultValue>
      <description>url</description>
      <id>8a7d7de4-14ae-4d4f-8122-78388d246176</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
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

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
