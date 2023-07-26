class NDClient {
  private token: string | null;

  public constructor() {
    this.token = null;
  }

  public setToken(token: string) {
    this.token = token;
  }
}

export const ndClient = new NDClient();
