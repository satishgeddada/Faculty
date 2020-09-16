<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For updating the single conference details</description>
   <name>Updating the single conference details</name>
   <tag></tag>
   <elementGuidId>8f4b9dcb-3c79-4940-9dab-4b76e6701c7a</elementGuidId>
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
   <restUrl>${URL}/faculty/insertConferenceDetails?employeeID=5AB4B781DE61438B95F0D20F678088F5&amp;insertData=%7B%22typeOfconferencename%22%3A%22Conference%22%2C%22titleConference%22%3A%22sdfsdf%22%2C%22internationalNational%22%3A%22National%22%2C%22organizedBy%22%3A%22svce%22%2C%22typeOfIssn%22%3A%22ISSN%22%2C%22issnIsbnNo%22%3A%22sdfsdf%22%2C%22typeOfissue%22%3A%22SCOPUS%22%2C%22issueNo%22%3A%22sdfsdf%22%2C%22confMonYear%22%3A%222020-04-30T18%3A30%3A00.000Z%22%2C%22depart%22%3A%22201961010135731679305002%22%2C%22conferenceID%22%3Anull%7D&amp;deptId=undefined&amp;state=forConferenceInsert</restUrl>
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
      <id>fd0c61ba-9699-40f6-864f-a564bc1e36e2</id>
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
