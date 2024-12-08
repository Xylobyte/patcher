/*
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//  Specification Extensions
//   ^x-
export type IExtensionName = `x-${string}`;
export type IExtensionType = any;
export type ISpecificationExtension = {
    [extensionName: IExtensionName]: IExtensionType;
};

export class SpecificationExtension implements ISpecificationExtension {
    [extensionName: IExtensionName]: any;

    static isValidExtension(extensionName: string): boolean {
        return /^x-/.test(extensionName);
    }

    getExtension(extensionName: string): any {
        if (!SpecificationExtension.isValidExtension(extensionName)) {
            throw new Error(
                `Invalid specification extension: '${extensionName}'. Extensions must start with prefix 'x-`
            );
        }
        if (this[extensionName as IExtensionName]) {
            return this[extensionName as IExtensionName];
        }
        return null;
    }
    addExtension(extensionName: string, payload: any): void {
        if (!SpecificationExtension.isValidExtension(extensionName)) {
            throw new Error(
                `Invalid specification extension: '${extensionName}'. Extensions must start with prefix 'x-`
            );
        }
        this[extensionName as IExtensionName] = payload;
    }
    listExtensions(): string[] {
        const res: string[] = [];
        for (const propName in this) {
            if (Object.prototype.hasOwnProperty.call(this, propName)) {
                if (SpecificationExtension.isValidExtension(propName)) {
                    res.push(propName);
                }
            }
        }
        return res;
    }
}