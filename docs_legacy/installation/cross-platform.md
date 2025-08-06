---
last_updated: 2025-05-30
layout: default
title: "Cross Platform"
description: "Documentation for Cross Platform"
---

[AIR-3][AIS-3][BPC-3][RES-3]

last_updated: 2025-05-30

# Cross-Platform Installation Guide

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)



Anya now supports cross-platform installation using the React SDK, making it easier to deploy and run on any operating system. The React SDK is the primary solution for web and desktop. For mobile, use the native Android/iOS SDKs (see platform-specific docs).


## Prerequisites

### Web/Desktop (React SDK)

- Node.js (v18+ recommended)
- npm or yarn

### Mobile

- Android: See [Android SDK Guide](../mobile/ANDROID.md)
- iOS: See [iOS SDK Guide](../mobile/IOS.md)


## Installing Anya (React SDK)

```bash
# Install dependencies
npm install anya-react-sdk
# or
yarn add anya-react-sdk
```


## Verifying Installation

Import and use the SDK in your React app:

```javascript
import { AnyaProvider } from 'anya-react-sdk';

function App() {
  return (
    <AnyaProvider>
      {/* your app */}
    </AnyaProvider>
  );
}
```


## Configuration

Refer to the [React SDK documentation](../web/REACT_SDK.md) for configuration options and usage examples.


## Running Anya

Run your React app as usual:

```bash
npm start
# or
yarn start
```


## Development Setup

For development, use standard React/Node.js tools. See the [React SDK README](../web/REACT_SDK.md) for details.


## Troubleshooting

See the [React SDK Troubleshooting Guide](../web/REACT_SDK_TROUBLESHOOTING.md) for common issues and solutions.


## Next Steps

- [Quick Start Guide](../getting-started/quick-start)
- [API Reference](../api/)
- [Security Best Practices](../security/)

*Last updated: 2025-06-02*

## See Also

- [Related Document](#related-document)

