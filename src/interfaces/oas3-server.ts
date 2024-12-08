/*
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

import { ServerObject, ServerVariableObject } from './oas3-common.ts';
import { IExtensionName, IExtensionType } from './oas3-ext.ts';

// Server & Server Variable
export class Server implements ServerObject {
    url: string;
    description?: string;
    variables: { [v: string]: ServerVariable };
    [k: IExtensionName]: IExtensionType;

    constructor(url: string, desc?: string) {
        this.url = url;
        this.description = desc;
        this.variables = {};
    }
    addVariable(name: string, variable: ServerVariable): void {
        this.variables[name] = variable;
    }
}

export class ServerVariable implements ServerVariableObject {
    enum?: string[] | boolean[] | number[];
    default: string | boolean | number;
    description?: string;
    [k: IExtensionName]: IExtensionType;

    constructor(
        defaultValue: string | boolean | number,
        enums?: string[] | boolean[] | number[],
        description?: string
    ) {
        this.default = defaultValue;
        this.enum = enums;
        this.description = description;
    }
}