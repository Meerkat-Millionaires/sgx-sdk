import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey } from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh';

export interface QuoteVerifyParamsFields {
  timestamp: BN;
}

export interface QuoteVerifyParamsJSON {
  timestamp: string;
}

export class QuoteVerifyParams {
  readonly timestamp: BN;

  constructor(fields: QuoteVerifyParamsFields) {
    this.timestamp = fields.timestamp;
  }

  static layout(property?: string) {
    return borsh.struct([borsh.i64('timestamp')], property);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new QuoteVerifyParams({
      timestamp: obj.timestamp,
    });
  }

  static toEncodable(fields: QuoteVerifyParamsFields) {
    return {
      timestamp: fields.timestamp,
    };
  }

  toJSON(): QuoteVerifyParamsJSON {
    return {
      timestamp: this.timestamp.toString(),
    };
  }

  static fromJSON(obj: QuoteVerifyParamsJSON): QuoteVerifyParams {
    return new QuoteVerifyParams({
      timestamp: new BN(obj.timestamp),
    });
  }

  toEncodable() {
    return QuoteVerifyParams.toEncodable(this);
  }
}
