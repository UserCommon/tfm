const config = {
    content: [
      "./src/**/*.{html,js,svelte,ts}",
      // Load Flowbite Svelte
      "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
    ],
  
    plugins: [
      // Enable the Flowbite Svelte plugin
      require('flowbite/plugin')
    ],
  
    darkMode: 'class',
  
    theme: {
      extend: {
        colors: {
          // Set the basic color palette - feel free to customize this
          // More example palettes available at:
          // https://flowbite-svelte.com/docs/pages/quickstart#Configuration
          primary: {
            50: '#FFF5F2',
            100: '#FFF1EE',
            200: '#FFE4DE',
            300: '#FFD5CC',
            400: '#FFBCAD',
            500: '#FE795D',
            600: '#EF562F',
            700: '#EB4F27',
            800: '#CC4522',
            900: '#A5371B'
          },
        }
      }
    }
  };
  
  module.exports = config;