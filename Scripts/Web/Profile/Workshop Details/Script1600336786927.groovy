import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://oecfrontend.vercel.app/')

WebUI.setText(findTestObject('Web/Login/input_Faculty Login_username'), 'DEMO051020')

WebUI.setEncryptedText(findTestObject('Web/Login/input_Faculty Login_password'), 'Sp03FKARx74=')

WebUI.click(findTestObject('Web/Login/button_Login'))

WebUI.click(findTestObject('Web/Profile/Workshop Details/a_Profile'))

WebUI.click(findTestObject('Object Repository/span_ MORE'))

WebUI.click(findTestObject('Web/Profile/Workshop Details/button_Workshop Details'))

WebUI.click(findTestObject('Web/Profile/Workshop Details/button_ADD'))

WebUI.setText(findTestObject('Web/Profile/Workshop Details/input_Organized By_orgnizedBy'), 'JNTU')

WebUI.setText(findTestObject('Web/Profile/Workshop Details/input_Duration_duration'), '1 Day')

WebUI.setText(findTestObject('Web/Profile/Workshop Details/input_Workshop Month  Year_monYear'), '01/01/2020')

WebUI.setText(findTestObject('Web/Profile/Workshop Details/input_Place_place'), 'Hyderabad')

WebUI.click(findTestObject('Web/Profile/Workshop Details/button_Save Workshop'))

WebUI.closeBrowser()

