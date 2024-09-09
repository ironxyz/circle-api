# \PullCryptoPaymentsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_crypto_payment**](PullCryptoPaymentsApi.md#create_crypto_payment) | **POST** /v1/payments/crypto | Create a crypto payment
[**presign**](PullCryptoPaymentsApi.md#presign) | **GET** /v1/payments/presign | Get a typed message for signing



## create_crypto_payment

> models::CreateCryptoPaymentResponse create_crypto_payment(crypto_payment_creation_request)
Create a crypto payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_payment_creation_request** | Option<[**CryptoPaymentCreationRequest**](CryptoPaymentCreationRequest.md)> |  |  |

### Return type

[**models::CreateCryptoPaymentResponse**](CreateCryptoPaymentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## presign

> models::CreateCryptoPaymentResponse1 presign(end_user_address, payment_intent_id, amount, currency)
Get a typed message for signing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**end_user_address** | **String** | User wallet address | [required] |
**payment_intent_id** | **uuid::Uuid** | Payment intent id which is from the create payment intent endpoint response | [required] |
**amount** | Option<**String**> |  |  |
**currency** | Option<**String**> | Only support USD at the moment |  |

### Return type

[**models::CreateCryptoPaymentResponse1**](CreateCryptoPaymentResponse_1.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

