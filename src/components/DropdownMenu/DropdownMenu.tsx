import * as RadixDropdownMenu from "@radix-ui/react-dropdown-menu";
import { DropdownMenuProps } from "./types";

function DropdownMenu(props: DropdownMenuProps) {
    const { children, items, onClick } = props;
    return (
        <RadixDropdownMenu.Root>
            <RadixDropdownMenu.Trigger asChild>
                {children}
            </RadixDropdownMenu.Trigger>
            <RadixDropdownMenu.Portal>
                <RadixDropdownMenu.Content className='min-w-[100px] bg-white rounded-md p-[5px] shadow-[0px_10px_38px_-10px_rgba(22,_23,_24,_0.35),_0px_10px_20px_-15px_rgba(22,_23,_24,_0.2)] will-change-[opacity,transform] data-[side=top]:animate-slideDownAndFade data-[side=right]:animate-slideLeftAndFade data-[side=bottom]:animate-slideUpAndFade data-[side=left]:animate-slideRightAndFade'>
                    {items?.map((item) => (
                        <RadixDropdownMenu.Item onClick={() => onClick?.(item)}>
                            {item}
                        </RadixDropdownMenu.Item>
                    ))}
                </RadixDropdownMenu.Content>
            </RadixDropdownMenu.Portal>
        </RadixDropdownMenu.Root>
    );
}

export default DropdownMenu;
