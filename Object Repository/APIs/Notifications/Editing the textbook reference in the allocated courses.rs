<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For editing the textbook reference in the allocated courses</description>
   <name>Editing the textbook reference in the allocated courses</name>
   <tag></tag>
   <elementGuidId>2f8e2940-1862-41f6-889f-a6e44c4d1a89</elementGuidId>
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
   <restUrl>${URL}/faculty/updateRefData?updateData=%7B%22clientID%22%3A%227BA54C5313174AE58FCB45B1A40F608A%22%2C%22updatedBy%22%3A%22100%22%2C%22setActive%22%3A%22Y%22%2C%22createdBy%22%3A%22100%22%2C%22author%22%3A%228779hkhk%22%2C%22employeeID%22%3A%2264DF070D8B6B11E98B0925647F276BC6%22%2C%22courseBookId%22%3A%2220208264515989%22%2C%22courseID%22%3A%22201961010181722142801645%22%2C%22bookName%22%3A%22sdkdfkhk%22%2C%22orgID%22%3A%220%22%2C%22%24%24hashKey%22%3A%22object%3A2455%22%7D&amp;tableName=booksTable</restUrl>
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
      <id>c352d034-e941-4ee2-bc14-1267dcb20abf</id>
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
