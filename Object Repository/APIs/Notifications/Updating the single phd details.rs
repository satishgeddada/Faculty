<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For updating the single phd details</description>
   <name>Updating the single phd details</name>
   <tag></tag>
   <elementGuidId>21417ca8-41f6-4236-92d6-aca21afc4331</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/faculty/insertPhdDetails?employeeID=64DF070D8B6B11E98B0925647F276BC6&amp;insertData=%7B%22researchTitle%22%3A%22sdfsdf%22%2C%22university%22%3A%22svu%22%2C%22registeredYear%22%3A%222015%22%2C%22phdstatus%22%3A%22Completed%22%2C%22dateAward%22%3A%222015-07-05T18%3A30%3A00.000Z%22%2C%22supervisorName%22%3A%22sdfdssdf%22%2C%22supervisionAff%22%3A%22svu%22%2C%22deptId%22%3A%2220196101013404918557388%22%2C%22phdDetailsId%22%3A%2220208263520251%22%7D&amp;state=forPhdDetailsInsert</restUrl>
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
      <id>9df0bd7c-d9de-4e9c-8c39-0a8eb50792f4</id>
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
