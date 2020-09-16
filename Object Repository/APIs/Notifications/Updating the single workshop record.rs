<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For updating the single workshop record</description>
   <name>Updating the single workshop record</name>
   <tag></tag>
   <elementGuidId>a3938a35-29a5-45b9-b49a-8a78b129f3a8</elementGuidId>
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
   <restUrl>${URL}/faculty/insertWorkShops?employeeID=5AB4B781DE61438B95F0D20F678088F5&amp;insertData=%7B%22typeOfWorkshopname%22%3A%22FDP%22%2C%22workshopname%22%3A%22sdfsd%22%2C%22orgnizedBy%22%3A%22AITS%22%2C%22duration%22%3A%225days%22%2C%22monYear%22%3A%222020-02-29T18%3A30%3A00.000Z%22%2C%22place%22%3A%22hyd%22%2C%22typeOfworkshop%22%3A%22National%22%2C%22workdeptId%22%3A%7B%22departmentName%22%3A%2205%22%2C%22hodName%22%3A%22T.N.%20Ranganadham%22%2C%22hodID%22%3A%22466DDFD65934495A98466D0F5191E02C%22%2C%22departmentPrimaryId%22%3A%22201961010135731679305002%22%2C%22departmentdes%22%3A%22CSE%22%7D%2C%22workID%22%3A%22202073113424771%22%7D&amp;state=forWorkshopInsert</restUrl>
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
      <id>3b1a6f93-0fce-4877-9ce9-c80469d2011f</id>
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
