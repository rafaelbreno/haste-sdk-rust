import { isBrowser } from '../util/environmentCheck';
import createAuth0Client, { Auth0Client } from '@auth0/auth0-spa-js';
import { HasteClientConfiguration } from '../config/hasteClientConfiguration';

export class HasteClient {
  private auth0Client: Auth0Client;
  private configuration: HasteClientConfiguration;

  private constructor(configuration: HasteClientConfiguration, auth0Client: Auth0Client) {
    this.configuration = configuration;
    this.auth0Client = auth0Client;
  }

  public static async build(clientId: string, domain = 'haste-production.us.auth0.com') {
    if (!isBrowser()) throw new Error(`Haste client build may only be called from a browser based environment`);

    const auth0 = await createAuth0Client({
      domain: domain,
      client_id: clientId,
      audience: 'https://hastegame.api',
    });

    return new HasteClient(
      {
        domain: domain,
        clientId: clientId,
      },
      auth0,
    );
  }

  public logout() {
    return this.auth0Client.logout({
      returnTo: window.location.origin,
    });
  }

  public async getTokenSilently() {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-return
    return await this.auth0Client.getTokenSilently();
  }

  public async isAuthenticated() {
    return await this.auth0Client.isAuthenticated();
  }

  public async loginWithRedirect() {
    return await this.auth0Client.loginWithRedirect({
      redirect_uri: window.location.origin,
    });
  }

  public async handleRedirectCallback() {
    return await this.auth0Client.handleRedirectCallback();
  }
}
