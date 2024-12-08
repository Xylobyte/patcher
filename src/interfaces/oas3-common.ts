/*
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

import {
    IExtensionName,
    ISpecificationExtension,
    SpecificationExtension
} from './oas3-ext.ts';

export interface ServerObject extends ISpecificationExtension {
    url: string;
    description?: string;
    variables?: { [v: string]: ServerVariableObject };
}
export interface ServerVariableObject extends ISpecificationExtension {
    enum?: string[] | boolean[] | number[];
    default: string | boolean | number;
    description?: string;
}

export function getExtension(obj: ISpecificationExtension | undefined, extensionName: string): any {
    if (!obj) {
        return undefined;
    }
    if (SpecificationExtension.isValidExtension(extensionName)) {
        return obj[extensionName as IExtensionName];
    }
    return undefined;
}
export function addExtension(
    obj: ISpecificationExtension | undefined,
    extensionName: string,
    extension: any
): void {
    if (obj && SpecificationExtension.isValidExtension(extensionName)) {
        obj[extensionName as IExtensionName] = extension;
    }
}