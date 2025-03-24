import React from 'react';
import { View, Text } from 'react-native';

export const SilentLeaf = ({ metadata }: { metadata: string }) => (
  <View style={styles.container}>
    <Text style={styles.hash}>
      {sha256(metadata).slice(0, 12)}
    </Text>
    <TaprootIndicator />
  </View>
); 