import { Dimensions, Platform } from 'react-native';

const dim = Dimensions.get('window');
const isDevelopment = process.env.NODE_ENV === 'development';
if (isDevelopment) {
  console.warn({isDevelopment})
}

export default {
  window: dim,
  isSmallDevice: dim.width < 375,
  os: Platform.OS,
  isWeb: Platform.OS === 'web',
  isDevelopment: isDevelopment,
};
