<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For inserting the single journal publication</description>
   <name>Inserting the single journal publication</name>
   <tag></tag>
   <elementGuidId>788a3196-213e-4d7b-87aa-e4c43c997457</elementGuidId>
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
   <restUrl>${URL}/faculty/insertPaperPublications?employeeID=5AB4B781DE61438B95F0D20F678088F5&amp;insertData=%7B%22typeOfAuthor%22%3A%22Co-Author%22%2C%22titlePublication%22%3A%22sdfsdfsdfsdf%22%2C%22journalName%22%3A%22sdfsdf%22%2C%22typeOfPublication%22%3A%22National%22%2C%22pageNos%22%3A%22sdfsdfsdf%22%2C%22issnNo%22%3A%225445%22%2C%22monYear%22%3A%222020-06-30T18%3A30%3A00.000Z%22%2C%22typeOfIssue%22%3A%22SCOPUS%22%2C%22issueNo%22%3A%22sdfsdf%22%2C%22volumeNo%22%3A%22sdfsd%22%2C%22impactFactor%22%3A%22sdfsd%22%2C%22Hindex%22%3A%22esw%22%7D&amp;state=forPapersInsert</restUrl>
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
      <description>URL</description>
      <id>4d4f0e78-b205-4417-92ac-24e05d9dab2c</id>
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
