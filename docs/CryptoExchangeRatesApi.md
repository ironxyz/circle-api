# \CryptoExchangeRatesApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_exchange_rates**](CryptoExchangeRatesApi.md#get_exchange_rates) | **GET** /v1/exchange/rates/{trading-pair} | Get a exchange rate



## get_exchange_rates

> models::GetExchangeRatesResponse get_exchange_rates(trading_pair)
Get a exchange rate

Fetch the current rates for the specified trading pair. The trading pair is defined by a base currency followed by a quote currency. The response contains buy and sell rates denominated in the quote currency. **Exchange rate is an estimate only and is subject to change by the time you submit the actual request.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trading_pair** | **String** |  | [required] |

### Return type

[**models::GetExchangeRatesResponse**](GetExchangeRatesResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

